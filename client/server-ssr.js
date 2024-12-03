import fs from 'node:fs';
import express from 'express';

const isProduction = process.argv[3] === 'release';

const ssrManifest = isProduction
    ? await fs.promises.readFile('./client/dist/client/.vite/ssr-manifest.json', 'utf-8')
    : undefined

const app = express();

app.use(express.json());


let vite;
if (!isProduction) {
    const { createServer } = await import('vite');
    vite = await createServer({
        server: { middlewareMode: true },
        appType: 'custom',
        base: '/src',
    });
    app.use(vite.middlewares);
} else {
    const compression = (await import('compression')).default
    app.use(compression())
}

app.post('/render', async (req, res) => {
    const { url } = req.body;
    if (isProduction) {
        // Release
        try {
            let render = (await import('./dist/server/entry-server.js')).render
            const result = await render(url, ssrManifest);
            res.json(result);
        } catch (err) {
            console.error(err);
            res.status(500).json({ error: 'SSR failed' });
        }
    } else {
        // Debug
        // Always read fresh template in development
        try {
            let template = await fs.promises.readFile('./index.html', 'utf-8')
            template = await vite.transformIndexHtml(url, template)
            let render = (await vite.ssrLoadModule('./src/entry-server.tsx')).render
            const result = await render(url, ssrManifest);
            res.json(result);
        } catch (err) {
            console.error(err);
            res.status(500).json({ error: 'SSR failed' });
        }
    }
});

const PORT = process.argv[2] || 3001;
app.listen(PORT, () => {
    console.log(`SSR server is running on port ${PORT}`);
});