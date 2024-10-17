import IntegrationsList from "./integrations_listing";

interface IntegrationsHeaderProps {
    integrationsSelected:boolean;

}

export function IntegrationsHeader({integrationsSelected}: IntegrationsHeaderProps) {
    
    
    return (
        <div className="integrations_header">
            <span>Selected {integrationsSelected}</span>
            <span>Integration</span>
            <span>Version</span>
            <span>Action</span>
        </div>
    );
}

export default IntegrationsHeader;