import fs from 'node:fs';
import express from 'express';
import { render } from './dist/server/entry-server.js';

const ssr_manifest = await fs.promises.readFile('./client/dist/client/.vite/ssr-manifest.json', 'utf-8');
const app = express();

app.use(express.json());

app.post('/render', async (req, res) => {
    const { url } = req.body;
    // Release
    try {
        const result = await render(url, ssr_manifest);
        res.json(result);
    } catch (err) {
        console.error(err);
        res.status(500).json({ error: 'SSR failed' });
    }
});

const PORT = process.argv[2] || 3001;
app.listen(PORT, () => {
    console.log(`SSR server is running on port ${PORT}`);
});