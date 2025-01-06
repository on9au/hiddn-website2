import './index.css'
import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import Router from './router'

// ReactDOM.hydrateRoot(
//   document.getElementById('root')!,
//   <React.StrictMode>
//     <BrowserRouter>
//         <Router />
//     </BrowserRouter>
//   </React.StrictMode>
// )
ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <BrowserRouter>
            <Router />
        </BrowserRouter>
    </React.StrictMode>
)