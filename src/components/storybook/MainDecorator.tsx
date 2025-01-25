import { API } from "@/lib/api";
import { Parameters } from "storybook/internal/types";
import Layout from "../_layout/Layout";
import Root from "../_layout/Root";

const mockAPI: API = {
  getShoppingLists: async () => [],
  getItemsAndSuggestions: async () => [],
  subscribeToEvents: () => () => {},
  updateDocument: async () => {},
};

interface MainDecoratorProps {
  parameters: Parameters;
  children: React.ReactNode;
}
export default function MainDecorator({
  parameters,
  children,
}: MainDecoratorProps) {
  const { pageLayout, layout } = parameters;
  switch (pageLayout ?? layout) {
    case "fullscreen":
    case "page":
      return (
        <Root api={mockAPI}>
          <Layout>{children}</Layout>
        </Root>
      );
    default:
      return <Root api={mockAPI}>{children}</Root>;
  }
}
