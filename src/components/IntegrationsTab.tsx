import { useState } from 'react';
import IntegrationsContainerHeader from './utils/integrations/integ_cont_header';
import IntegrationsContainer from './utils/integrations/integ_cont_list';

const IntegrationsTab = () => {

  return (
    <div>
      <IntegrationsContainerHeader />
      <IntegrationsContainer />
    </div>
  );
};

export default IntegrationsTab;
