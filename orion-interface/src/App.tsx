import { useState } from "react";
import "./App.css";
import ExtensionList from "./components/extensions_listing";

function App() {
  const [name, setName] = useState("");
  const lastVer = "1.0.0"; // Define the lastVer variable


  return (
    <div className="container">
      <div className="header orion">
        <div>Orion</div>
        <div className="row">Last Update <p className="lastVersion">{lastVer}</p></div>
      </div>
      
      <div className="container" style={{ display: 'flex', flexDirection: 'row' }}>
        <div className="settings-sidebar" style={{ display: 'flex', flexDirection: 'column', flex: 1 }}>
          <ul>
            <li>Plataformas</li>
            <li>Integrações</li>
            <li>Funcionalidades</li>
            <li>Geral</li>
            <li>Interface</li>
            <li>Notificações</li>
            <li>Jogos GOG.com</li>
            <li>Instalando, Atualizando</li>
            <li>Recursos de Jogos</li>
          </ul>
        </div>


        <div className="right_container" style={{ flex: 1 }}>
          <div>
            <div>
              <ExtensionList />
            </div>
          </div>
        </div>


      </div>
    </div>

  );
}

export default App;
