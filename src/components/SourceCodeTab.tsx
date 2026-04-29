const SourceCodeTab = () => {
  return (
    <section className="source-code-tab">
      <h2>Source Code</h2>
      <p>Follow the active repository and its upstream integrations.</p>
      <ul>
        <li>
          <a href="https://github.com/GOG-Orion/orion-interface" target="_blank" rel="noreferrer">
            Orion Interface
          </a>
        </li>
        <li>
          <a
            href="https://github.com/GOG-Orion/galaxy-integration-steam"
            target="_blank"
            rel="noreferrer"
          >
            Steam integration
          </a>
        </li>
        <li>
          <a href="https://github.com/GOG-Orion/.github" target="_blank" rel="noreferrer">
            Orion organization docs
          </a>
        </li>
      </ul>
    </section>
  );
};
  
export default SourceCodeTab;
