import { getMenuText } from '../languages.ts';

export function IntegrationsContainerHeader() {
    const selectedLanguage = "ENG";
    const menuText = getMenuText(selectedLanguage);

    return (
        <div className="integrations_tab_header">
            <span>{menuText.int_head}</span>
            <span>{menuText.int_search}</span>
        </div>
    );
}

export default IntegrationsContainerHeader;
