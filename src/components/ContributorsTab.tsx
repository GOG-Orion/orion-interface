const contributors = [
  {
    name: "Vitor (v-Kaleb) Guttler",
    role: "Maintainer",
  },
  {
    name: "GOG-Orion community",
    role: "Integration testing and feedback",
  },
];

const ContributorsTab = () => {
  return (
    <section className="contributors-tab">
      <h2>Contributors</h2>
      <p>People and groups contributing to Orion.</p>
      <ul>
        {contributors.map((contributor) => (
          <li key={contributor.name}>
            <strong>{contributor.name}</strong> - {contributor.role}
          </li>
        ))}
      </ul>
    </section>
  );
};
  
export default ContributorsTab;
