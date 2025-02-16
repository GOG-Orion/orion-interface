import React from "react";
import ReactDOM from "react-dom";
import App from "./App";
import { LanguageProvider } from "./components/LanguageContext";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <LanguageProvider>
      <App />
    </LanguageProvider>
  </React.StrictMode>
);
