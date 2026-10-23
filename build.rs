#[cfg(target_os = "windows")]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/blockops_app_icon.ico");
    res.set("FileDescription", "BlockOps Static IP Manager");
    res.set("ProductName", "BlockOps Static IP Manager 3.0.0");
    res.set("CompanyName", "BlockOps Mining");
    res.set("FileVersion", "3.0.0.0");
    res.set("ProductVersion", "3.0.0.0");
    res.set_manifest(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>"#,
    );
    res.compile().expect("failed to embed Windows resource");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
