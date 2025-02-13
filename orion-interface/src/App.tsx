import "./App.css";
import IntegrationsContainerHeader from "./components/integrations_header";
import IntegrationsList from "./components/integrations_listing";
import SettingsSidebar from "./components/settings_sidebar";

function App() {

  return (
    <div>
      <div className="orionHeader">
      <h1>Orion</h1>
      </div>
      <div>
        <div className="settings-content">
          <SettingsSidebar />
          <div className="container_integrations">
            <div>
              <IntegrationsContainerHeader />
              <IntegrationsList />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
