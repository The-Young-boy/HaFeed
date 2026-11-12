// Generates src/bindings.rs (widgets WinRT API, toasts). Run from the crate root:
// cargo run --manifest-path tools/gen/Cargo.toml
fn main() {
    windows_bindgen::bindgen([
        "--in", "default", "winmd/Microsoft.Windows.Widgets.winmd",
        "--out", "src/bindings.rs",
        "--flat",
        "--implement",
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProvider", "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceProvider",
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnnouncementInvokedTarget",
        "Microsoft.Windows.Widgets.Providers.IWidgetProvider", "Microsoft.Windows.Widgets.Providers.IWidgetProvider2",
        "--filter", "Microsoft.Windows.Widgets.Feeds.Providers", "Microsoft.Windows.Widgets.Providers", "Microsoft.Windows.Widgets.Notifications", "Microsoft.Windows.Widgets.WidgetSize",
        "Windows.Storage.Streams.RandomAccessStreamReference", "Windows.Storage.Streams.IRandomAccessStream", "Windows.Storage.Streams.IRandomAccessStreamReference", "Windows.Storage.Streams.IRandomAccessStreamWithContentType",
        "Windows.UI.Notifications.ToastNotificationManager", "Windows.UI.Notifications.ToastNotification", "Windows.UI.Notifications.ToastNotifier",
        "Windows.Data.Xml.Dom.XmlDocument", "Windows.Foundation.Uri", "Windows.Storage.Streams", "Windows.Storage.StorageStreamTransaction", "Windows.UI.Notifications.ScheduledToastNotification",
        "Windows.Networking.Connectivity", "Windows.Networking.HostName", "Windows.Networking.HostNameType", "Windows.Networking.DomainNameType",
    ]);
}
