import React from 'react'
import ReactDOMServer from 'react-dom/server'
import Router from './router'
import { StaticRouter } from 'react-router-dom/server'

export function render() {
  const html = ReactDOMServer.renderToString(
    <React.StrictMode>
        <StaticRouter location={"/"}>
            <Router />
        </StaticRouter>
      {/* <RouterProvider router={router} /> */}
    </React.StrictMode>
  )
  return { html }
}