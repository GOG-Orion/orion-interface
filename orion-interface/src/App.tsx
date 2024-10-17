import "./App.css";
import IntegrationsHeader from "./components/integrations_header";
import IntegrationsList from "./components/integrations_listing";
import SettingsSidebar from "./components/settings_sidebar";

function App() {

  return (
    <div className="orionHeader">
      <h1>Orion</h1>
      <div className="settings-container" style={{ display: 'flex', flexDirection: 'row' }}>
          <div className="settings-content">
            <SettingsSidebar />
            <div className="integrations_main" style={{ flex: 1 }}>
              <IntegrationsHeader />
              <div>
                <IntegrationsList />
              </div>
            </div>
          </div>
      </div>
    </div>
  );
}

export default App;
