import '@mantine/core/styles.css';

import "./App.css";
import { MantineProvider } from "@mantine/core";
import { Content } from './Content';
import { Layout } from './layout/Layout';
import api, { APIContext } from "./api";

function App() {
  return (
    <MantineProvider defaultColorScheme="dark">
      <APIContext.Provider value={api}>
        <Layout>
          <Content />
        </Layout>
      </APIContext.Provider>
    </MantineProvider>
  )
}

export default App;
