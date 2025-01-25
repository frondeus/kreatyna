import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import './i18next';
import { ErrorPage } from "./layout/Error";
import { ErrorBoundary } from 'react-error-boundary';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ErrorBoundary FallbackComponent={ErrorPage}>
      <App />
    </ErrorBoundary>
  </React.StrictMode>,
);
