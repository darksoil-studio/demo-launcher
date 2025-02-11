import { AdminWebsocket } from "@holochain/client";
import { createContext } from "@lit/context";

export const adminWebsocketContext =
  createContext<AdminWebsocket>("adminWebsocket");
