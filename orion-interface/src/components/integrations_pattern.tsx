import { useState } from "react";

interface IntegrationsPatternProps {
    isEnabled: boolean;
    isDisabled: boolean;
    onEnableAction: () => void;
    onDisableAction: () => void;
    name: string;
    integrationImage: string;
}

export function IntegrationsPattern ({isEnabled, isDisabled, onEnableAction, onDisableAction, name, integrationImage}: IntegrationsPatternProps) {
    const [extensionName, setExtensionName] = useState(name);
    
    const handleCheckboxChange = () => {
        if (isDisabled) {
            onDisableAction();
        } else if (isEnabled) {
             onEnableAction(); 
            }
    };
    
    // Define the lastVer variable
    const lastVer = "1.0.0";
    // Generate image path based on extensionImage prop
    const integrationImagePath = `../src/assets/${integrationImage}Logo.svg`;

    return (
        <div className="integration-item">
                <input type="checkbox" />
                <img src={integrationImagePath} className="logo" alt="Extension logo"/>
                <span className="extProps">
                    <span className="extProps Name">{extensionName}</span>
                    <span className="extProps Ver">{lastVer}</span>
                    <button onClick={handleCheckboxChange}>
                        {isEnabled ? "Disable" : "Verify"}
                    </button>
                </span>
        </div>

    );
}

export default IntegrationsPattern;