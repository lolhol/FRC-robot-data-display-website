import ExampleRoute from "./Example/ExampleRoute";

export const ROUTE_MAP: Record<string, (data: any) => Promise<void>> = {
  ExampleRoute,
};
