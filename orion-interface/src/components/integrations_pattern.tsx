import { useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import { confirm, message } from "@tauri-apps/api/dialog";

interface IntegrationsPatternProps {
    isEnabled: boolean;
    name: string;
    integrationImage: string;
    updateSelectedCount: (delta: number) => void;
}

export function IntegrationsPattern ({
     isEnabled,
     updateSelectedCount,
     name,
     integrationImage
    }: IntegrationsPatternProps) {
        const [extensionName, setExtensionName] = useState(name);
        const [isChecked, setIsChecked] = useState(false);

        const handleCheckboxChange = () => {
            setIsChecked(!isChecked);
            updateSelectedCount(isChecked ? -1 : 1);
        };

        const handleVerifyClick = async () => {
            const confirmation = await confirm("Do you want to start verification?", "Verification Started");
            if (confirmation) {
              try {
                await invoke("verify_action");
                await message("Verification Started", { title: "Info" });
              } catch (error) {
                console.error("Verification failed:", error);
                await message("Verification failed", { title: "Error", type: "error" });
              }
            }
          };

    // Define the lastVer variable
    const lastVer = "1.0.0";
    // Generate image path based on extensionImage prop
    const integrationImagePath = `../src/assets/${integrationImage}Logo.svg`;


    return (
        <div className="integration-item">
            <span className="integration-item Props">
                <input type="checkbox" checked={isChecked} onChange={handleCheckboxChange} />
                <img className="integration-item Logo" src={integrationImagePath} alt="Extension logo"/>
                <span className="integration-item Name">{extensionName}</span>
            </span>
            <span className="integration-item Props">
                <span className="integration-item Ver">{lastVer}</span>
                <button onClick={handleVerifyClick}>
                    {isEnabled ? "Disable" : "Verify"}
                </button>
            </span>
        </div>
    );
}

export default IntegrationsPattern;