import {useState} from 'react';
import { getMenuText } from '../languages.ts';

export function IntegrationsContainerHeader() {
    const [selectedLanguage, setSelectedLanguage] = useState("ENG");

    const handleLanguageChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        setSelectedLanguage(event.target.value);
    };

    const menuText = getMenuText(selectedLanguage);

    return (
        <div className="integrations_tab_header">
            <span>{menuText.int_head}</span>
            <span>{menuText.int_search}</span>
        </div>
    );
}

export default IntegrationsContainerHeader;
