import "./App.css";
import { useState } from "react";
import IntegrationsHeader from "./components/integrations_header";
import IntegrationsList from "./components/integrations_listing";
import SettingsSidebar from "./components/settings_sidebar";

function App() {
  const [selectedCount, setSelectedCount] = useState(0);

  const updateSelectedCount = (delta: number) => {
    setSelectedCount((prevCount) => prevCount + delta);
  };

  return (
    <div className="orionHeader">
      <h1>Orion</h1>
      <div className="container_settings">
        <div className="settings-content">
          <SettingsSidebar />
          <div className="container_integrations">
            <div>
              <IntegrationsHeader selectedCount={selectedCount} />
              <IntegrationsList updateSelectedCount={updateSelectedCount} />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
