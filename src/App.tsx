import Content from "@components/_layout/Content";
import Layout from "@components/_layout/Layout";
import Root from "@components/_layout/Root";
import "@mantine/core/styles.css";
import "./App.css";

export default function App() {
  return (
    <Root>
      <Layout>
        <Content />
      </Layout>
    </Root>
  );
}
