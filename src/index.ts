import { AdminWebsocket, AppClient, AppWebsocket } from "@holochain/client";
import { css, html, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";
import "@shoelace-style/shoelace/dist/components/spinner/spinner.js";
import "@tnesh-stack/elements/dist/elements/display-error.js";
import "@tnesh-stack/elements/dist/elements/app-client-context.js";
import "@darksoil-studio/happs-zome/dist/elements/happs-context.js";
import "@darksoil-studio/file-storage-zome/dist/elements/file-storage-context.js";
import { msg } from "@lit/localize";
import { provide } from "@lit/context";
import { adminWebsocketContext } from "./context";
import { notifyError, sharedStyles } from "@tnesh-stack/elements";
import { openHappStore } from "./commands";
import { SlButton } from "@shoelace-style/shoelace";
import "./running-apps.js";

@customElement("demo-launcher")
export class DemoLauncher extends LitElement {
  @state()
  _loading = true;
  @state()
  _view = { view: "main" };
  @state()
  _error: unknown | undefined;

  @provide({ context: adminWebsocketContext })
  _adminWebsocket!: AdminWebsocket;

  _client!: AppClient;

  async firstUpdated() {
    try {
      this._adminWebsocket = await AdminWebsocket.connect();
      this._client = await AppWebsocket.connect();
    } catch (e: unknown) {
      this._error = e;
    } finally {
      this._loading = false;
    }
  }

  render() {
    if (this._loading) {
      return html`<div
        class="row"
        style="flex: 1; height: 100%; align-items: center; justify-content: center;"
      >
        <sl-spinner style="font-size: 2rem"></sl-spinner>
      </div>`;
    }

    if (this._error) {
      return html`
        <div
          style="flex: 1; height: 100%; align-items: center; justify-content: center;"
        >
          <display-error
            .error=${this._error}
            .headline=${msg("Error connecting to holochain")}
          >
          </display-error>
        </div>
      `;
    }

    return html`
      <app-client-context .client=${this._client}>
        <happs-context role="main">
          <file-storage-context role="main" zome="file_storage_gateway">
            <div class="column" style="flex: 1">
              <div class="row top-bar">
                <span class="title" style="flex: 1"
                  >${msg("Demo Launcher")}</span
                >
                <sl-button
                  @click=${async (e: CustomEvent) => {
                    (e.target as SlButton).loading = true;
                    try {
                      await openHappStore();
                    } catch (e) {
                      notifyError(msg("Error opening the hApp Store."));
                      console.error(e);
                    }
                    (e.target as SlButton).loading = false;
                  }}
                  >${msg("hApp Store")}
                </sl-button>
              </div>
              <running-apps style="flex: 1"></running-apps>
            </div>
          </file-storage-context>
        </happs-context>
      </app-client-context>
    `;
  }

  static styles = [
    sharedStyles,
    css`
      :host {
        display: flex;
        flex: 1;
      }
      .top-bar {
        align-items: center;
        box-shadow: rgba(149, 157, 165, 1) 2px 2px 4px;
        padding: 16px;
        height: 40px;
      }
    `,
  ];
}
