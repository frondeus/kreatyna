import '@mantine/core/styles.css';

import "./App.css";
import Content from './components/_layout/Content';
import Layout from './components/_layout/Layout';
import api from "@lib/api";
import Root from './components/_layout/Root';

export default function App() {
  return (
    <Root api={api}>
      <Layout>
        <Content />
      </Layout>
    </Root>
  )
}

