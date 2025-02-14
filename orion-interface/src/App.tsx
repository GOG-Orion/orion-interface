import "./App.css";
import SettingsSidebar from "./components/MenuSidebar";

function App() {

  return (
    <div>
      <div className="orionHeader">
      <h1>Orion</h1>
      </div>
      <div>
        <div> {/* This is the main container for the settings page */}
          <SettingsSidebar />
        </div>
      </div>
    </div>
  );
}

export default App;
