import { AdminWebsocket, AppInfo, AppStatusFilter } from "@holochain/client";
import { sharedStyles, wrapPathInSvg } from "@tnesh-stack/elements";
import { css, html, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";
import "@shoelace-style/shoelace/dist/components/spinner/spinner.js";
import "@tnesh-stack/elements/dist/elements/display-error.js";
import { msg } from "@lit/localize";
import { consume } from "@lit/context";
import { mdiInformationOutline } from "@mdi/js";

import { adminWebsocketContext } from "./context";

@customElement("running-apps")
export class RunningApps extends LitElement {
  @state()
  _loading = true;
  @state()
  _view = { view: "main" };
  @state()
  _error: unknown | undefined;

  @consume({ context: adminWebsocketContext })
  _adminWebsocket!: AdminWebsocket;

  @state()
  apps!: Array<AppInfo>;

  async firstUpdated() {
    try {
      const apps = await this._adminWebsocket.listApps({
        status_filter: AppStatusFilter.Running,
      });
      this.apps = apps.filter((app) => app.installed_app_id !== "happ-store");
    } catch (e: unknown) {
      this._error = e;
    } finally {
      this._loading = false;
    }
  }

  renderApp(app: AppInfo) {
    return html`<div class="column" style="gap: 8px">
      ${app.installed_app_id}
    </div>`;
  }

  renderApps(apps: Array<AppInfo>) {
    if (apps.length === 0) {
      return html` <div
        class="column center-content"
        style="gap: 16px; flex: 1"
      >
        <sl-icon
          .src=${wrapPathInSvg(mdiInformationOutline)}
          style="color: grey; height: 64px; width: 64px;"
        ></sl-icon>
        <span class="placeholder"
          >${msg("You haven't installed any hApps yet.")}</span
        >
        <span class="placeholder"
          >${msg(
            "Go to the hApp Store to explore and download new hApps.",
          )}</span
        >
      </div>`;
    }

    return html`<div class="row" style="gap: 16px; flex-wrap: wrap">
      ${apps.map((app) => this.renderApp(app))}
    </div>`;
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

    return this.renderApps(this.apps);
  }

  static styles = [
    sharedStyles,
    css`
      :host {
        display: flex;
      }
    `,
  ];
}
