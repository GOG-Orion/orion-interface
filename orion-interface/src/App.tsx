import "./App.css";
import IntegrationsHeader from "./components/integrations_header";
import IntegrationsList from "./components/integrations_listing";
import SettingsSidebar from "./components/settings_sidebar";

function App() {

  return (
    <div className="orionHeader">
      <h1>Orion</h1>
      <div className="container_settings">
        <div className="settings-content">
          <SettingsSidebar />
          <div className="container_integrations">
            <div>
              <IntegrationsHeader />
              <IntegrationsList />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
