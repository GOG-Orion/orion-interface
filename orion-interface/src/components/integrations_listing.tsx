import integrationsList from './integrations.json';
import { IntegrationsPattern } from './integrations_pattern';

interface IntegrationsListProps {
    updateSelectedCount: (delta: number) => void;
  }

function IntegrationsList({ updateSelectedCount }: IntegrationsListProps) {

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
                    updateSelectedCount={updateSelectedCount}
                />
            ))}
        </div>
    );
}

export default IntegrationsList;