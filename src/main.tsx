import React from "react"
import ReactDOM from "react-dom/client"
import App from "./App.tsx"
import "./App.css"  // <-- must be here, and path/case must match

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
