import "./App.css";
import MenuSidebar from "./components/MenuSidebar";

function App() {

  return (
    <div>
      <div className="orionHeader">
      <h1>Orion</h1>
      </div>
      <div>
        <div> {/* This is the main container for the settings page */}
          <MenuSidebar />
        </div>
      </div>
    </div>
  );
}

export default App;
