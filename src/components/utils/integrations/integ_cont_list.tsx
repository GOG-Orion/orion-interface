// IntegrationsList.tsx
import integrationsList from "../integrations.json";
import { IntegrationsContainerPattern } from "./integ_cont_pattern";

function IntegrationsContainerList() {
  return (
    <div>
      {integrationsList.map((integration, index) => (
        <IntegrationsContainerPattern
          key={index}
          name={integration.name}
          integrationImage={integration.image}
        />
      ))}
    </div>
  );
}

export default IntegrationsContainerList;
