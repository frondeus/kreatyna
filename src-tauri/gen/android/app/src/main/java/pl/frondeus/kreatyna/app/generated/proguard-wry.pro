# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class pl.frondeus.kreatyna.app.* {
  native <methods>;
}

-keep class pl.frondeus.kreatyna.app.WryActivity {
  public <init>(...);

  void setWebView(pl.frondeus.kreatyna.app.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class pl.frondeus.kreatyna.app.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class pl.frondeus.kreatyna.app.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void setAutoPlay(...);
  void setUserAgent(...);
  void evalScript(...);
}

-keep class pl.frondeus.kreatyna.app.RustWebChromeClient,pl.frondeus.kreatyna.app.RustWebViewClient {
  public <init>(...);
}