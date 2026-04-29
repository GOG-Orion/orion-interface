// MenuSidebar.tsx
import { useState } from 'react';
import { useLanguage } from './LanguageContext.tsx';
import languages from './utils/languages.json';
import IntegrationsTab from './IntegrationsTab.tsx';
import IntegrationsImage from './assets/integrations_link.svg';
import SourceCodeTab from './SourceCodeTab.tsx';
import SourceCodeImage from './assets/sourcecode_github.svg';
import ContributorsTab from './ContributorsTab.tsx';
import ContributorsImage from './assets/contributors_group.svg';
import ConfigurationsTab from './ConfigurationsTab.tsx';
import ConfigurationsImage from './assets/configuration_gear.svg';
import { ORION_VERSION } from './utils/version';



interface MenuSidebarProps {}

export function MenuSidebar({}: MenuSidebarProps) {
  const {selectedLanguage, menuText, setLanguage} = useLanguage();
  const [selectedTab, setSelectedTab] = useState("menu_item01"); // Default tab


  return (
    <div className="menu-container">
      {/* Sidebar */}
      <div className="menu-sidebar">
        <h3>{menuText.menu_head}</h3>
        <ul>
          <li
            onClick={() => setSelectedTab("menu_item01")}
            className={selectedTab === "menu_item01" ? "active" : ""}
          >
            <span className="menu_items">
              <img
                src={IntegrationsImage}
                alt="link"
                style={{ width: "18px", height: "18px"}}
              />
              {menuText.menu_item01}
            </span>
          </li>
          <li
            onClick={() => setSelectedTab("menu_item02")}
            className={selectedTab === "menu_item02" ? "active" : ""}
          >
            <span className="menu_items">
            <img
                src={SourceCodeImage}
                alt="link"
                style={{ width: "18px", height: "18px"}}
              />
              {menuText.menu_item02}
              </span>
          </li>
          <li
            onClick={() => setSelectedTab("menu_item03")}
            className={selectedTab === "menu_item03" ? "active" : ""}
          >
            <span className="menu_items">
            <img
                src={ContributorsImage}
                alt="link"
                style={{ width: "18px", height: "18px"}}
              />
              {menuText.menu_item03}
              </span>
          </li>
        </ul>
        <ul>
          <li
            onClick={() => setSelectedTab("menu_item04")}
            className={selectedTab === "menu_item04" ? "active" : ""}
          >
            <span className="menu_items">
            <img
                src={ConfigurationsImage}
                alt="link"
                style={{ width: "18px", height: "18px"}}
              />
              {menuText.menu_item04}
              </span>
          </li>
        </ul>
        {/* Language Selector */}
        <ul>
          <li>
            <span className="menu_items">
            <select 
              value={selectedLanguage} 
              onChange={(e) => setLanguage(e.target.value)}
            >
              {languages.map((language) => (
                <option key={language.lang} value={language.lang}>
                  {language.lang}
                </option>
              ))}
            </select>
            </span>
          </li>
        </ul>
        <div className="orion_ver" title="Orion Version">
          <div>
            <span>{ORION_VERSION}</span>
            <span> - Alpha</span>
          </div>
        </div>
      </div>
      {/* Tabs Contents */}
      <div className="tab-content">
        {selectedTab === "menu_item01" && <IntegrationsTab /> }
        {selectedTab === "menu_item02" && <SourceCodeTab /> }
        {selectedTab === "menu_item03" && <ContributorsTab /> }
        {selectedTab === "menu_item04" && <ConfigurationsTab /> }
      </div>
    </div>
  );
}

export default MenuSidebar;
