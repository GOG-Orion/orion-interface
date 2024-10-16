import {useState} from 'react';

interface SettingsSidebarProps {

}



export function SettingsSidebar({}: SettingsSidebarProps) {
    const [] = useState();

    const handleClick = () => {
        // Implement handleClick logic, to change tab on click
    };


    return (
    <div className="settings-sidebar" style={{ display: 'flex', flexDirection: 'column', flex: 1 }}>
        <h3>General</h3>
        <ul>
            <li>Integrations</li>
            <li>Functionalities</li>
            <li>Interface</li>
            <li>Notifications</li>
            <li>Contributors</li>
        </ul>
        <ul>
            <li>Config.</li>
        </ul>
        <div className="orion version" title="Orion Version">
            <div>
            <span>0.0.2 </span>
            <span>- Alpha</span>
            </div>
        </div>
    </div>
    );
}

export default SettingsSidebar;