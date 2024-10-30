interface IntegrationsHeaderProps {
    selectedCount: number;
}

export function IntegrationsHeader({selectedCount}:IntegrationsHeaderProps) {
    return (
        <div className="integrations_header">
            <span>Selected {selectedCount}</span>
            <span>Integration</span>
            <span>Version</span>
            <span>Action</span>
        </div>
    );
}

export default IntegrationsHeader;