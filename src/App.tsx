import '@mantine/core/styles.css';

import "./App.css";
import { AppShell, MantineProvider } from "@mantine/core";

function App() {
  return (
    <MantineProvider defaultColorScheme="dark">
      <AppShell>
        <AppShell.Main>
          Hello
          </AppShell.Main>
      </AppShell>
    </MantineProvider>
  )
}

export default App;
