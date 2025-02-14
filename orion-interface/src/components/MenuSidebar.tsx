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
            <span>0.0.6</span>
            <span> - Alpha</span>
          </div>
        </div>
      </div>
      {/* Tabs Contents */}
      <div className="tab-content">
        {selectedTab === "menu_item01" && <IntegrationsTab /> }
        {selectedTab === "menu_item02" && <SourceCodeTab /> }
        {selectedTab === "menu_item03" && (
          <div>
            <h2>{menuText.menu_item03}</h2>
            {/* This renders your item03 */}
            <p>Details for {menuText.menu_item03} content.</p>
          </div>
        )}
        {selectedTab === "menu_item04" && (
          <div>
            <h2>{menuText.menu_item04}</h2>
            {/* This renders your item04 */}
            <p>Details for {menuText.menu_item04} content.</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default MenuSidebar;
