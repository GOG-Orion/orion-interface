import { useLanguage } from "../../LanguageContext";

export function IntegrationsContainerHeader() {
    const { menuText } = useLanguage();

    return (
        <div className="integrations_tab_header">
            <span>{menuText.int_head}</span>
            <span>{menuText.int_search}</span>
        </div>
    );
}

export default IntegrationsContainerHeader;
