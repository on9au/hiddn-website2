import express from 'express';
import { render } from './dist/server/entry-server.js';

const app = express();

app.use(express.json());

app.post('/render', async (req, res) => {
    const { url } = req.body;
    try {
        const result = await render(url);
        res.json(result);
    } catch (err) {
        console.error(err);
        res.status(500).json({ error: 'SSR failed' });
    }
});

const PORT = 3001;
app.listen(PORT, () => {
    console.log(`SSR server is running on port ${PORT}`);
});