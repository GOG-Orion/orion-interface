import integrationsList from './integrations.json';
import { IntegrationsPattern } from './integrations_pattern';

function IntegrationsList() {
    return (
        <div>
            {integrationsList.map((integration, index) => (
                <IntegrationsPattern
                    key={index}
                    isEnabled={false}
                    isDisabled={false}
                    onEnableAction={() => {
                        // Implement install func call logic
                    }}
                    onDisableAction={() => {
                        // Implement skip install func call logic
                    }}
                    name={integration.name}
                    integrationImage={integration.image}
                />
            ))}
        </div>
    );
}

export default IntegrationsList;