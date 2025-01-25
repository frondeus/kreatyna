import Content from "@components/_layout/Content";
import Layout from "@components/_layout/Layout";
import Root from "@components/_layout/Root";
import api from "@lib/api";
import "@mantine/core/styles.css";
import "./App.css";

export default function App() {
  return (
    <Root api={api}>
      <Layout>
        <Content />
      </Layout>
    </Root>
  );
}
