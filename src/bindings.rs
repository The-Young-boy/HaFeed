#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnnouncementActionKind(pub i32);
impl AnnouncementActionKind {
    pub const Shown: Self = Self(0);
    pub const Engaged: Self = Self(1);
}
impl windows_core::imp::TypeKind for AnnouncementActionKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for AnnouncementActionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.Widgets.Notifications.AnnouncementActionKind;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.AnnouncementActionKind",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnnouncementTextColor(pub i32);
impl AnnouncementTextColor {
    pub const Default: Self = Self(0);
    pub const Dark: Self = Self(1);
    pub const Light: Self = Self(2);
    pub const Accent: Self = Self(3);
    pub const Good: Self = Self(4);
    pub const Warning: Self = Self(5);
    pub const Attention: Self = Self(6);
}
impl windows_core::imp::TypeKind for AnnouncementTextColor {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for AnnouncementTextColor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.Widgets.Notifications.AnnouncementTextColor;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.AnnouncementTextColor",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Buffer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Buffer,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IBuffer
);
impl Buffer {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capacity)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Create(capacity: u32) -> windows_core::Result<Self> {
        Self::IBufferFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                capacity,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IBufferFactory<R, F: FnOnce(&IBufferFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Buffer, IBufferFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBufferStatics<R, F: FnOnce(&IBufferStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Buffer, IBufferStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Buffer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBuffer>();
}
unsafe impl windows_core::Interface for Buffer {
    type Vtable = <IBuffer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBuffer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Buffer {
    const NAME: &'static str = "Windows.Storage.Streams.Buffer";
}
unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: Self = Self(0);
    pub const BigEndian: Self = Self(1);
}
impl windows_core::imp::TypeKind for ByteOrder {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ByteOrder {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.ByteOrder;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.ByteOrder");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomQueryParametersRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CustomQueryParametersRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CustomQueryParametersRequestedArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for CustomQueryParametersRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICustomQueryParametersRequestedArgs>();
}
unsafe impl windows_core::Interface for CustomQueryParametersRequestedArgs {
    type Vtable = <ICustomQueryParametersRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICustomQueryParametersRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomQueryParametersRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.CustomQueryParametersRequestedArgs";
}
unsafe impl Send for CustomQueryParametersRequestedArgs {}
unsafe impl Sync for CustomQueryParametersRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomQueryParametersUpdateOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CustomQueryParametersUpdateOptions,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CustomQueryParametersUpdateOptions {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CustomQueryParameters(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomQueryParameters)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(
        feedproviderdefinitionid: &windows_core::HSTRING,
        customqueryparameters: &windows_core::HSTRING,
    ) -> windows_core::Result<Self> {
        Self::ICustomQueryParametersUpdateOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(customqueryparameters),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn ICustomQueryParametersUpdateOptionsFactory<
        R,
        F: FnOnce(&ICustomQueryParametersUpdateOptionsFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CustomQueryParametersUpdateOptions,
            ICustomQueryParametersUpdateOptionsFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CustomQueryParametersUpdateOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICustomQueryParametersUpdateOptions>();
}
unsafe impl windows_core::Interface for CustomQueryParametersUpdateOptions {
    type Vtable = <ICustomQueryParametersUpdateOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICustomQueryParametersUpdateOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomQueryParametersUpdateOptions {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.CustomQueryParametersUpdateOptions";
}
unsafe impl Send for CustomQueryParametersUpdateOptions {}
unsafe impl Sync for CustomQueryParametersUpdateOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DataReader,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IDataReader
);
windows_core::imp::required_hierarchy!(DataReader, IClosable);
impl DataReader {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn UnconsumedBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnconsumedBufferLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ByteOrder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetByteOrder)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InputStreamOptions(&self) -> windows_core::Result<InputStreamOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InputStreamOptions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInputStreamOptions)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ReadByte(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadByte)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ReadBytes)(
                windows_core::Interface::as_raw(self),
                value.len().try_into().unwrap(),
                value.as_mut_ptr(),
            )
            .ok()
        }
    }
    pub fn ReadBuffer(&self, length: u32) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBuffer)(
                windows_core::Interface::as_raw(self),
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReadBoolean(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBoolean)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadGuid)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadInt16(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt16)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadInt32(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt32)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadInt64(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt64)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadUInt16(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt16)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadUInt32(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt32)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadUInt64(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt64)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadSingle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadSingle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadDouble(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDouble)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadString)(
                windows_core::Interface::as_raw(self),
                codeunitcount,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadDateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDateTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadTimeSpan(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadTimeSpan)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadAsync)(
                windows_core::Interface::as_raw(self),
                count,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateDataReader<P0>(inputstream: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IInputStream>,
    {
        Self::IDataReaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDataReader)(
                windows_core::Interface::as_raw(this),
                inputstream.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn FromBuffer<P0>(buffer: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IBuffer>,
    {
        Self::IDataReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBuffer)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDataReader>();
}
unsafe impl windows_core::Interface for DataReader {
    type Vtable = <IDataReader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataReader {
    const NAME: &'static str = "Windows.Storage.Streams.DataReader";
}
unsafe impl Send for DataReader {}
unsafe impl Sync for DataReader {}
pub type DataReaderLoadOperation = windows_future::IAsyncOperation<u32>;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataWriter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DataWriter,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IDataWriter
);
windows_core::imp::required_hierarchy!(DataWriter, IClosable);
impl DataWriter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            DataWriter,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn UnstoredBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnstoredBufferLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ByteOrder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetByteOrder)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteByte(&self, value: u8) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteByte)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBytes)(
                windows_core::Interface::as_raw(self),
                value.len().try_into().unwrap(),
                value.as_ptr(),
            )
            .ok()
        }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBuffer)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
            )
            .ok()
        }
    }
    pub fn WriteBufferRange<P0>(
        &self,
        buffer: P0,
        start: u32,
        count: u32,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBufferRange)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                start,
                count,
            )
            .ok()
        }
    }
    pub fn WriteBoolean(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBoolean)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteGuid(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteGuid)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteInt16(&self, value: i16) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteInt16)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteInt32(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteInt32)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteInt64(&self, value: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteInt64)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteUInt16(&self, value: u16) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteUInt16)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteUInt32(&self, value: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteUInt32)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteUInt64(&self, value: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteUInt64)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteSingle(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteSingle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteDouble(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteDouble)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteDateTime(&self, value: windows_time::DateTime) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteDateTime)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteTimeSpan(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteTimeSpan)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteString)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MeasureString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MeasureString)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn StoreAsync(&self) -> windows_core::Result<DataWriterStoreOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StoreAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateDataWriter<P0>(outputstream: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IOutputStream>,
    {
        Self::IDataWriterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDataWriter)(
                windows_core::Interface::as_raw(this),
                outputstream.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDataWriterFactory<R, F: FnOnce(&IDataWriterFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataWriter, IDataWriterFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDataWriter>();
}
unsafe impl windows_core::Interface for DataWriter {
    type Vtable = <IDataWriter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataWriter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriter";
}
unsafe impl Send for DataWriter {}
unsafe impl Sync for DataWriter {}
pub type DataWriterStoreOperation = windows_future::IAsyncOperation<u32>;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedAnalyticsInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedAnalyticsInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedAnalyticsInfoReportedArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnalyticsJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FeedAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedAnalyticsInfoReportedArgs>();
}
unsafe impl windows_core::Interface for FeedAnalyticsInfoReportedArgs {
    type Vtable = <IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IFeedAnalyticsInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedAnalyticsInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.FeedAnalyticsInfoReportedArgs";
}
unsafe impl Send for FeedAnalyticsInfoReportedArgs {}
unsafe impl Sync for FeedAnalyticsInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedAnnouncement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedAnnouncement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedAnnouncement {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetId)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn PrimaryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPrimaryText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrimaryText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn SecondaryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondaryText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSecondaryText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSecondaryText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn LightModeIconUri(&self) -> windows_core::Result<Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LightModeIconUri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetLightModeIconUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetLightModeIconUri)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn DarkModeIconUri(&self) -> windows_core::Result<Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DarkModeIconUri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDarkModeIconUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDarkModeIconUri)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn PrimaryTextColor(&self) -> windows_core::Result<AnnouncementTextColor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryTextColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPrimaryTextColor(&self, value: AnnouncementTextColor) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrimaryTextColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn SecondaryTextColor(&self) -> windows_core::Result<AnnouncementTextColor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondaryTextColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSecondaryTextColor(&self, value: AnnouncementTextColor) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSecondaryTextColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CustomAccessibilityText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomAccessibilityText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCustomAccessibilityText(
        &self,
        value: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomAccessibilityText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsSecondaryTextSubtle(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSecondaryTextSubtle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsSecondaryTextSubtle(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSecondaryTextSubtle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ShowBadgeIfUserNotEngaged(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowBadgeIfUserNotEngaged)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetShowBadgeIfUserNotEngaged(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetShowBadgeIfUserNotEngaged)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpirationTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExpirationTime(&self, value: windows_time::DateTime) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetExpirationTime)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Duration(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duration)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDuration)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CreateInstance<P3, P4>(
        id: &windows_core::HSTRING,
        primarytext: &windows_core::HSTRING,
        secondarytext: &windows_core::HSTRING,
        lightmodeicon: P3,
        darkmodeicon: P4,
    ) -> windows_core::Result<Self>
    where
        P3: windows_core::Param<Uri>,
        P4: windows_core::Param<Uri>,
    {
        Self::IFeedAnnouncementFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(id),
                core::mem::transmute_copy(primarytext),
                core::mem::transmute_copy(secondarytext),
                lightmodeicon.param().abi(),
                darkmodeicon.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFeedAnnouncementFactory<
        R,
        F: FnOnce(&IFeedAnnouncementFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FeedAnnouncement, IFeedAnnouncementFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FeedAnnouncement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedAnnouncement>();
}
unsafe impl windows_core::Interface for FeedAnnouncement {
    type Vtable = <IFeedAnnouncement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedAnnouncement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedAnnouncement {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Notifications.FeedAnnouncement";
}
unsafe impl Send for FeedAnnouncement {}
unsafe impl Sync for FeedAnnouncement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedAnnouncementInvokedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedAnnouncementInvokedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedAnnouncementInvokedArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn AnnouncementId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnnouncementId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActionKind(&self) -> windows_core::Result<AnnouncementActionKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FeedAnnouncementInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedAnnouncementInvokedArgs>();
}
unsafe impl windows_core::Interface for FeedAnnouncementInvokedArgs {
    type Vtable = <IFeedAnnouncementInvokedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedAnnouncementInvokedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedAnnouncementInvokedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Notifications.FeedAnnouncementInvokedArgs";
}
unsafe impl Send for FeedAnnouncementInvokedArgs {}
unsafe impl Sync for FeedAnnouncementInvokedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedDisabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedDisabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedDisabledArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FeedDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedDisabledArgs>();
}
unsafe impl windows_core::Interface for FeedDisabledArgs {
    type Vtable = <IFeedDisabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedDisabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedDisabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedDisabledArgs";
}
unsafe impl Send for FeedDisabledArgs {}
unsafe impl Sync for FeedDisabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedEnabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedEnabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedEnabledArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FeedEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedEnabledArgs>();
}
unsafe impl windows_core::Interface for FeedEnabledArgs {
    type Vtable = <IFeedEnabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedEnabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedEnabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedEnabledArgs";
}
unsafe impl Send for FeedEnabledArgs {}
unsafe impl Sync for FeedEnabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedErrorInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedErrorInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedErrorInfoReportedArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FeedErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedErrorInfoReportedArgs>();
}
unsafe impl windows_core::Interface for FeedErrorInfoReportedArgs {
    type Vtable = <IFeedErrorInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedErrorInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedErrorInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.FeedErrorInfoReportedArgs";
}
unsafe impl Send for FeedErrorInfoReportedArgs {}
unsafe impl Sync for FeedErrorInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedManager,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IFeedManager
);
windows_core::imp::required_hierarchy!(FeedManager, IFeedManager2);
impl FeedManager {
    pub fn GetEnabledFeedProviders(
        &self,
    ) -> windows_core::Result<windows_core::Array<FeedProviderInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetEnabledFeedProviders)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<FeedProviderInfo>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn SetCustomQueryParameters<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CustomQueryParametersUpdateOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomQueryParameters)(
                windows_core::Interface::as_raw(self),
                options.param().abi(),
            )
            .ok()
        }
    }
    pub fn SendMessageToContent(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFeedManager2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SendMessageToContent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
    pub fn TryShowAnnouncement<P2>(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        announcement: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<FeedAnnouncement>,
    {
        let this = &windows_core::Interface::cast::<IFeedManager2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).TryShowAnnouncement)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                announcement.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetDefault() -> windows_core::Result<Self> {
        Self::IFeedManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFeedManagerStatics<R, F: FnOnce(&IFeedManagerStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FeedManager, IFeedManagerStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FeedManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedManager>();
}
unsafe impl windows_core::Interface for FeedManager {
    type Vtable = <IFeedManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedManager";
}
unsafe impl Send for FeedManager {}
unsafe impl Sync for FeedManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedMessageReceivedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedMessageReceivedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedMessageReceivedArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FeedMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedMessageReceivedArgs>();
}
unsafe impl windows_core::Interface for FeedMessageReceivedArgs {
    type Vtable = <IFeedMessageReceivedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedMessageReceivedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedMessageReceivedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedMessageReceivedArgs";
}
unsafe impl Send for FeedMessageReceivedArgs {}
unsafe impl Sync for FeedMessageReceivedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedProviderDisabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedProviderDisabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedProviderDisabledArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FeedProviderDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedProviderDisabledArgs>();
}
unsafe impl windows_core::Interface for FeedProviderDisabledArgs {
    type Vtable = <IFeedProviderDisabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedProviderDisabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedProviderDisabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderDisabledArgs";
}
unsafe impl Send for FeedProviderDisabledArgs {}
unsafe impl Sync for FeedProviderDisabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedProviderEnabledArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedProviderEnabledArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedProviderEnabledArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FeedProviderEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedProviderEnabledArgs>();
}
unsafe impl windows_core::Interface for FeedProviderEnabledArgs {
    type Vtable = <IFeedProviderEnabledArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedProviderEnabledArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedProviderEnabledArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderEnabledArgs";
}
unsafe impl Send for FeedProviderEnabledArgs {}
unsafe impl Sync for FeedProviderEnabledArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedProviderInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedProviderInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedProviderInfo {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn EnabledFeedDefinitionIds(
        &self,
    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).EnabledFeedDefinitionIds)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for FeedProviderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedProviderInfo>();
}
unsafe impl windows_core::Interface for FeedProviderInfo {
    type Vtable = <IFeedProviderInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedProviderInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedProviderInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedProviderInfo";
}
unsafe impl Send for FeedProviderInfo {}
unsafe impl Sync for FeedProviderInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedResourceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedResourceRequest,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedResourceRequest {
    pub fn Uri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMethod(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMethod)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Content(&self) -> windows_core::Result<IRandomAccessStreamReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRandomAccessStreamReference>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for FeedResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceRequest>();
}
unsafe impl windows_core::Interface for FeedResourceRequest {
    type Vtable = <IFeedResourceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedResourceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedResourceRequest {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceRequest";
}
unsafe impl Send for FeedResourceRequest {}
unsafe impl Sync for FeedResourceRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedResourceRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedResourceRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedResourceRequestedArgs {
    pub fn FeedProviderDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedProviderDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FeedDefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeedDefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Request(&self) -> windows_core::Result<FeedResourceRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Response(&self) -> windows_core::Result<FeedResourceResponse> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Response)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResponse<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedResourceResponse>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResponse)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for FeedResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceRequestedArgs>();
}
unsafe impl windows_core::Interface for FeedResourceRequestedArgs {
    type Vtable = <IFeedResourceRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedResourceRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedResourceRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceRequestedArgs";
}
unsafe impl Send for FeedResourceRequestedArgs {}
unsafe impl Sync for FeedResourceRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedResourceResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FeedResourceResponse,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FeedResourceResponse {
    pub fn Content(&self) -> windows_core::Result<IRandomAccessStreamReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Headers(
        &self,
    ) -> windows_core::Result<
        windows_collections::IIterable<
            windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>,
        >,
    > {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Headers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetHeaders<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<
                windows_collections::IIterable<
                    windows_collections::IKeyValuePair<
                        windows_core::HSTRING,
                        windows_core::HSTRING,
                    >,
                >,
            >,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeaders)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReasonPhrase(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReasonPhrase)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CreateInstance<P0>(
        content: P0,
        reasonphrase: &windows_core::HSTRING,
        statuscode: i32,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IRandomAccessStreamReference>,
    {
        Self::IFeedResourceResponseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                content.param().abi(),
                core::mem::transmute_copy(reasonphrase),
                statuscode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFeedResourceResponseFactory<
        R,
        F: FnOnce(&IFeedResourceResponseFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FeedResourceResponse,
            IFeedResourceResponseFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FeedResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFeedResourceResponse>();
}
unsafe impl windows_core::Interface for FeedResourceResponse {
    type Vtable = <IFeedResourceResponse as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFeedResourceResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FeedResourceResponse {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.FeedResourceResponse";
}
unsafe impl Send for FeedResourceResponse {}
unsafe impl Sync for FeedResourceResponse {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileInputStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FileInputStream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IInputStream
);
windows_core::imp::required_hierarchy!(FileInputStream, IClosable);
impl FileInputStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadAsync)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileInputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputStream>();
}
unsafe impl windows_core::Interface for FileInputStream {
    type Vtable = <IInputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileInputStream";
}
unsafe impl Send for FileInputStream {}
unsafe impl Sync for FileInputStream {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: Self = Self(0);
    pub const OpenAlways: Self = Self(1);
    pub const CreateNew: Self = Self(2);
    pub const CreateAlways: Self = Self(3);
    pub const TruncateExisting: Self = Self(4);
}
impl windows_core::imp::TypeKind for FileOpenDisposition {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FileOpenDisposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Storage.Streams.FileOpenDisposition;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.FileOpenDisposition");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileOutputStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FileOutputStream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IOutputStream
);
windows_core::imp::required_hierarchy!(FileOutputStream, IClosable);
impl FileOutputStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteAsync)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileOutputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IOutputStream>();
}
unsafe impl windows_core::Interface for FileOutputStream {
    type Vtable = <IOutputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOutputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileOutputStream";
}
unsafe impl Send for FileOutputStream {}
unsafe impl Sync for FileOutputStream {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileRandomAccessStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FileRandomAccessStream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IRandomAccessStream
);
windows_core::imp::required_hierarchy!(
    FileRandomAccessStream,
    IClosable,
    IInputStream,
    IOutputStream
);
impl FileRandomAccessStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn OpenTransactedWriteAsync(
        filepath: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_future::IAsyncOperation<StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenTransactedWriteAsync)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(filepath),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Seek)(
                windows_core::Interface::as_raw(self),
                position,
            )
            .ok()
        }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRead)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanWrite)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    fn IFileRandomAccessStreamStatics<
        R,
        F: FnOnce(&IFileRandomAccessStreamStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FileRandomAccessStream,
            IFileRandomAccessStreamStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FileRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStream>();
}
unsafe impl windows_core::Interface for FileRandomAccessStream {
    type Vtable = <IRandomAccessStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileRandomAccessStream";
}
unsafe impl Send for FileRandomAccessStream {}
unsafe impl Sync for FileRandomAccessStream {}
windows_core::imp::define_interface!(
    IBuffer,
    IBuffer_Vtbl,
    0x905a0fe0_bc53_11df_8c49_001e4fc686da
);
impl windows_core::RuntimeType for IBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IBuffer");
}
windows_core::imp::interface_hierarchy!(
    IBuffer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IBuffer {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capacity)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capacity:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBufferFactory,
    IBufferFactory_Vtbl,
    0x71af914d_c10f_484b_bc50_14bc623b3a27
);
impl windows_core::RuntimeType for IBufferFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IBufferFactory");
}
#[repr(C)]
pub struct IBufferFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBufferStatics,
    IBufferStatics_Vtbl,
    0xe901e65b_d716_475a_a90a_af7229b1e741
);
impl windows_core::RuntimeType for IBufferStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IBufferStatics");
}
#[repr(C)]
pub struct IBufferStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IClosable,
    IClosable_Vtbl,
    0x30d5a829_7fa4_4026_83bb_d75bae4ea99e
);
impl windows_core::RuntimeType for IClosable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IClosable");
}
windows_core::imp::interface_hierarchy!(
    IClosable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IClosable {
    pub fn Close(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IClosable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentTypeProvider,
    IContentTypeProvider_Vtbl,
    0x97d098a5_3b99_4de9_88a5_e11d2f50c795
);
impl windows_core::RuntimeType for IContentTypeProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IContentTypeProvider");
}
windows_core::imp::interface_hierarchy!(
    IContentTypeProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IContentTypeProvider {
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
pub struct IContentTypeProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICustomQueryParametersRequestedArgs,
    ICustomQueryParametersRequestedArgs_Vtbl,
    0xdc2b0cd8_7936_5346_9371_b21484c7d859
);
impl windows_core::RuntimeType for ICustomQueryParametersRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersRequestedArgs",
    );
}
#[repr(C)]
pub struct ICustomQueryParametersRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICustomQueryParametersUpdateOptions,
    ICustomQueryParametersUpdateOptions_Vtbl,
    0x753f1177_4909_568a_b070_98a3139205ec
);
impl windows_core::RuntimeType for ICustomQueryParametersUpdateOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptions",
    );
}
#[repr(C)]
pub struct ICustomQueryParametersUpdateOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomQueryParameters: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICustomQueryParametersUpdateOptionsFactory,
    ICustomQueryParametersUpdateOptionsFactory_Vtbl,
    0x34e318cd_3884_53c0_911c_225f32228fae
);
impl windows_core::RuntimeType for ICustomQueryParametersUpdateOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.ICustomQueryParametersUpdateOptionsFactory",
    );
}
#[repr(C)]
pub struct ICustomQueryParametersUpdateOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDataReader,
    IDataReader_Vtbl,
    0xe2b50029_b4c1_4314_a4b8_fb813a2f275e
);
impl windows_core::RuntimeType for IDataReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataReader");
}
windows_core::imp::interface_hierarchy!(
    IDataReader,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IDataReader {
    pub fn UnconsumedBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnconsumedBufferLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ByteOrder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetByteOrder)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InputStreamOptions(&self) -> windows_core::Result<InputStreamOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InputStreamOptions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInputStreamOptions)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ReadByte(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadByte)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ReadBytes)(
                windows_core::Interface::as_raw(self),
                value.len().try_into().unwrap(),
                value.as_mut_ptr(),
            )
            .ok()
        }
    }
    pub fn ReadBuffer(&self, length: u32) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBuffer)(
                windows_core::Interface::as_raw(self),
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReadBoolean(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBoolean)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadGuid)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadInt16(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt16)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadInt32(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt32)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadInt64(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt64)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadUInt16(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt16)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadUInt32(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt32)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadUInt64(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt64)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadSingle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadSingle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadDouble(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDouble)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadString)(
                windows_core::Interface::as_raw(self),
                codeunitcount,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadDateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDateTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReadTimeSpan(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadTimeSpan)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadAsync)(
                windows_core::Interface::as_raw(self),
                count,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDataReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnconsumedBufferLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut UnicodeEncoding,
    ) -> windows_core::HRESULT,
    pub SetUnicodeEncoding:
        unsafe extern "system" fn(*mut core::ffi::c_void, UnicodeEncoding) -> windows_core::HRESULT,
    pub ByteOrder:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut ByteOrder) -> windows_core::HRESULT,
    pub SetByteOrder:
        unsafe extern "system" fn(*mut core::ffi::c_void, ByteOrder) -> windows_core::HRESULT,
    pub InputStreamOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut InputStreamOptions,
    ) -> windows_core::HRESULT,
    pub SetInputStreamOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        InputStreamOptions,
    ) -> windows_core::HRESULT,
    pub ReadByte:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ReadBytes:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReadBoolean:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ReadGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub ReadInt16:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ReadInt32:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReadInt64:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub ReadUInt16:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ReadUInt32:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadUInt64:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub ReadSingle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub ReadDouble:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ReadString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReadDateTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub ReadTimeSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDataReaderFactory,
    IDataReaderFactory_Vtbl,
    0xd7527847_57da_4e15_914c_06806699a098
);
impl windows_core::RuntimeType for IDataReaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataReaderFactory");
}
#[repr(C)]
pub struct IDataReaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDataReader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDataReaderStatics,
    IDataReaderStatics_Vtbl,
    0x11fcbfc8_f93a_471b_b121_f379e349313c
);
impl windows_core::RuntimeType for IDataReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataReaderStatics");
}
#[repr(C)]
pub struct IDataReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDataWriter,
    IDataWriter_Vtbl,
    0x64b89265_d341_4922_b38a_dd4af8808c4e
);
impl windows_core::RuntimeType for IDataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataWriter");
}
windows_core::imp::interface_hierarchy!(
    IDataWriter,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IDataWriter {
    pub fn UnstoredBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnstoredBufferLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnicodeEncoding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ByteOrder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetByteOrder)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteByte(&self, value: u8) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteByte)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBytes)(
                windows_core::Interface::as_raw(self),
                value.len().try_into().unwrap(),
                value.as_ptr(),
            )
            .ok()
        }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBuffer)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
            )
            .ok()
        }
    }
    pub fn WriteBufferRange<P0>(
        &self,
        buffer: P0,
        start: u32,
        count: u32,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBufferRange)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                start,
                count,
            )
            .ok()
        }
    }
    pub fn WriteBoolean(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBoolean)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteGuid(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteGuid)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteInt16(&self, value: i16) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteInt16)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteInt32(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteInt32)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteInt64(&self, value: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteInt64)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteUInt16(&self, value: u16) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteUInt16)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteUInt32(&self, value: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteUInt32)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteUInt64(&self, value: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteUInt64)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteSingle(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteSingle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteDouble(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteDouble)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteDateTime(&self, value: windows_time::DateTime) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteDateTime)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteTimeSpan(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).WriteTimeSpan)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WriteString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteString)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MeasureString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MeasureString)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn StoreAsync(&self) -> windows_core::Result<DataWriterStoreOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StoreAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDataWriter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnstoredBufferLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut UnicodeEncoding,
    ) -> windows_core::HRESULT,
    pub SetUnicodeEncoding:
        unsafe extern "system" fn(*mut core::ffi::c_void, UnicodeEncoding) -> windows_core::HRESULT,
    pub ByteOrder:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut ByteOrder) -> windows_core::HRESULT,
    pub SetByteOrder:
        unsafe extern "system" fn(*mut core::ffi::c_void, ByteOrder) -> windows_core::HRESULT,
    pub WriteByte: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub WriteBytes:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub WriteBufferRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
    ) -> windows_core::HRESULT,
    pub WriteBoolean:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WriteGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub WriteInt16: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub WriteInt32: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WriteInt64: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WriteUInt16:
        unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteUInt32:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WriteUInt64:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub WriteSingle:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub WriteDouble:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub WriteDateTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub WriteTimeSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub WriteString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub MeasureString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub StoreAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FlushAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDataWriterFactory,
    IDataWriterFactory_Vtbl,
    0x338c67c2_8b84_4c2b_9c50_7b8767847a1f
);
impl windows_core::RuntimeType for IDataWriterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataWriterFactory");
}
#[repr(C)]
pub struct IDataWriterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDataWriter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnalyticsInfoReportedArgs,
    IFeedAnalyticsInfoReportedArgs_Vtbl,
    0x3c0e3d65_ed47_5b8a_b650_39a7edf18942
);
impl windows_core::RuntimeType for IFeedAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnalyticsInfoReportedArgs",
    );
}
#[repr(C)]
pub struct IFeedAnalyticsInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AnalyticsJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnnouncement,
    IFeedAnnouncement_Vtbl,
    0xb88e8c2c_d251_5344_acc2_8cf9ba07ec15
);
impl windows_core::RuntimeType for IFeedAnnouncement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncement",
    );
}
#[repr(C)]
pub struct IFeedAnnouncement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PrimaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrimaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SecondaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSecondaryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LightModeIconUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLightModeIconUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DarkModeIconUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetDarkModeIconUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PrimaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub SetPrimaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub SecondaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub SetSecondaryTextColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AnnouncementTextColor,
    ) -> windows_core::HRESULT,
    pub CustomAccessibilityText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCustomAccessibilityText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsSecondaryTextSubtle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSecondaryTextSubtle:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShowBadgeIfUserNotEngaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowBadgeIfUserNotEngaged:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnnouncementFactory,
    IFeedAnnouncementFactory_Vtbl,
    0x22074243_46d8_5af2_8715_1c76d1cb774c
);
impl windows_core::RuntimeType for IFeedAnnouncementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementFactory",
    );
}
#[repr(C)]
pub struct IFeedAnnouncementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnnouncementInvokedArgs,
    IFeedAnnouncementInvokedArgs_Vtbl,
    0x70a48d98_323d_5f19_a1e1_b63fe36edbf2
);
impl windows_core::RuntimeType for IFeedAnnouncementInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Notifications.IFeedAnnouncementInvokedArgs",
    );
}
#[repr(C)]
pub struct IFeedAnnouncementInvokedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AnnouncementId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ActionKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AnnouncementActionKind,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedAnnouncementInvokedTarget,
    IFeedAnnouncementInvokedTarget_Vtbl,
    0x5d44ae2a_072c_4df9_9fe5_34d5d2e9ff63
);
impl windows_core::RuntimeType for IFeedAnnouncementInvokedTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnnouncementInvokedTarget",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedAnnouncementInvokedTarget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedAnnouncementInvokedTarget {
    pub fn OnAnnouncementInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedAnnouncementInvokedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnAnnouncementInvoked)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IFeedAnnouncementInvokedTarget {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Feeds.Providers.IFeedAnnouncementInvokedTarget";
}
pub trait IFeedAnnouncementInvokedTarget_Impl: windows_core::IUnknownImpl {
    fn OnAnnouncementInvoked(
        &self,
        args: windows_core::Ref<FeedAnnouncementInvokedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedAnnouncementInvokedTarget_Vtbl {
    pub const fn new<Identity: IFeedAnnouncementInvokedTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAnnouncementInvoked<
            Identity: IFeedAnnouncementInvokedTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedAnnouncementInvokedTarget_Impl::OnAnnouncementInvoked(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IFeedAnnouncementInvokedTarget,
                OFFSET,
            >(),
            OnAnnouncementInvoked: OnAnnouncementInvoked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedAnnouncementInvokedTarget as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IFeedAnnouncementInvokedTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnAnnouncementInvoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedDisabledArgs,
    IFeedDisabledArgs_Vtbl,
    0x95300612_aca7_53c0_9cf6_d803689132c1
);
impl windows_core::RuntimeType for IFeedDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedDisabledArgs",
    );
}
#[repr(C)]
pub struct IFeedDisabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedEnabledArgs,
    IFeedEnabledArgs_Vtbl,
    0xeff4b2d7_7347_5969_a77d_cac433f0fdae
);
impl windows_core::RuntimeType for IFeedEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedEnabledArgs",
    );
}
#[repr(C)]
pub struct IFeedEnabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedErrorInfoReportedArgs,
    IFeedErrorInfoReportedArgs_Vtbl,
    0x62de018c_161e_52d0_9dbe_aec106fb6600
);
impl windows_core::RuntimeType for IFeedErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedErrorInfoReportedArgs",
    );
}
#[repr(C)]
pub struct IFeedErrorInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ErrorJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedManager,
    IFeedManager_Vtbl,
    0x87df6a84_15aa_45cb_8911_5cafab57f723
);
impl windows_core::RuntimeType for IFeedManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedManager {
    pub fn GetEnabledFeedProviders(
        &self,
    ) -> windows_core::Result<windows_core::Array<FeedProviderInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetEnabledFeedProviders)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<FeedProviderInfo>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn SetCustomQueryParameters<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CustomQueryParametersUpdateOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomQueryParameters)(
                windows_core::Interface::as_raw(self),
                options.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFeedManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetEnabledFeedProviders: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCustomQueryParameters: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedManager2,
    IFeedManager2_Vtbl,
    0x5838300a_a069_455d_9943_ba078ada00d8
);
impl windows_core::RuntimeType for IFeedManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManager2",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedManager2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedManager2 {
    pub fn SendMessageToContent(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SendMessageToContent)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
    pub fn TryShowAnnouncement<P2>(
        &self,
        feedproviderdefinitionid: &windows_core::HSTRING,
        feeddefinitionid: &windows_core::HSTRING,
        announcement: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<FeedAnnouncement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).TryShowAnnouncement)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(feedproviderdefinitionid),
                core::mem::transmute_copy(feeddefinitionid),
                announcement.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFeedManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessageToContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TryShowAnnouncement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedManagerStatics,
    IFeedManagerStatics_Vtbl,
    0x4baf5174_77d6_5e2a_94ea_4f14ccdb1f2c
);
impl windows_core::RuntimeType for IFeedManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedManagerStatics",
    );
}
#[repr(C)]
pub struct IFeedManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedMessageReceivedArgs,
    IFeedMessageReceivedArgs_Vtbl,
    0x4ed6ecf9_4c74_5a0b_ae04_bef6dd776f8a
);
impl windows_core::RuntimeType for IFeedMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedMessageReceivedArgs",
    );
}
#[repr(C)]
pub struct IFeedMessageReceivedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProvider,
    IFeedProvider_Vtbl,
    0x7293a12b_0329_458d_ac25_5332be478fde
);
impl windows_core::RuntimeType for IFeedProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProvider {
    pub fn OnFeedProviderEnabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedProviderEnabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedProviderEnabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFeedProviderDisabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedProviderDisabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedProviderDisabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFeedEnabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedEnabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedEnabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFeedDisabled<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedDisabledArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFeedDisabled)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCustomQueryParametersRequested<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CustomQueryParametersRequestedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnCustomQueryParametersRequested)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IFeedProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedProvider";
}
pub trait IFeedProvider_Impl: windows_core::IUnknownImpl {
    fn OnFeedProviderEnabled(
        &self,
        args: windows_core::Ref<FeedProviderEnabledArgs>,
    ) -> windows_core::Result<()>;
    fn OnFeedProviderDisabled(
        &self,
        args: windows_core::Ref<FeedProviderDisabledArgs>,
    ) -> windows_core::Result<()>;
    fn OnFeedEnabled(&self, args: windows_core::Ref<FeedEnabledArgs>) -> windows_core::Result<()>;
    fn OnFeedDisabled(&self, args: windows_core::Ref<FeedDisabledArgs>)
    -> windows_core::Result<()>;
    fn OnCustomQueryParametersRequested(
        &self,
        args: windows_core::Ref<CustomQueryParametersRequestedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedProvider_Vtbl {
    pub const fn new<Identity: IFeedProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnFeedProviderEnabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedProviderEnabled(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        unsafe extern "system" fn OnFeedProviderDisabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedProviderDisabled(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        unsafe extern "system" fn OnFeedEnabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedEnabled(this, core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn OnFeedDisabled<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnFeedDisabled(this, core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn OnCustomQueryParametersRequested<
            Identity: IFeedProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedProvider_Impl::OnCustomQueryParametersRequested(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedProvider, OFFSET>(),
            OnFeedProviderEnabled: OnFeedProviderEnabled::<Identity, OFFSET>,
            OnFeedProviderDisabled: OnFeedProviderDisabled::<Identity, OFFSET>,
            OnFeedEnabled: OnFeedEnabled::<Identity, OFFSET>,
            OnFeedDisabled: OnFeedDisabled::<Identity, OFFSET>,
            OnCustomQueryParametersRequested: OnCustomQueryParametersRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IFeedProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnFeedProviderEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFeedProviderDisabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFeedEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFeedDisabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnCustomQueryParametersRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderAnalytics,
    IFeedProviderAnalytics_Vtbl,
    0xf6885791_3085_4bd7_9cb1_4f1354c3a687
);
impl windows_core::RuntimeType for IFeedProviderAnalytics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderAnalytics",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProviderAnalytics,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProviderAnalytics {
    pub fn OnAnalyticsInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedAnalyticsInfoReportedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnAnalyticsInfoReported)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFeedProviderAnalytics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnAnalyticsInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderDisabledArgs,
    IFeedProviderDisabledArgs_Vtbl,
    0x19b65aec_e01d_5e8c_ab5f_324212e7cd30
);
impl windows_core::RuntimeType for IFeedProviderDisabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderDisabledArgs",
    );
}
#[repr(C)]
pub struct IFeedProviderDisabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderEnabledArgs,
    IFeedProviderEnabledArgs_Vtbl,
    0x821fc9af_0de6_5a9b_9ae6_e179117b40e4
);
impl windows_core::RuntimeType for IFeedProviderEnabledArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderEnabledArgs",
    );
}
#[repr(C)]
pub struct IFeedProviderEnabledArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderErrors,
    IFeedProviderErrors_Vtbl,
    0x6611e00a_d86c_49a3_9381_b7da67ee62dc
);
impl windows_core::RuntimeType for IFeedProviderErrors {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderErrors",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProviderErrors,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProviderErrors {
    pub fn OnErrorInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedErrorInfoReportedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnErrorInfoReported)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFeedProviderErrors_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnErrorInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderInfo,
    IFeedProviderInfo_Vtbl,
    0x73c37049_3c03_5896_8532_f9dfdaeb723f
);
impl windows_core::RuntimeType for IFeedProviderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderInfo",
    );
}
#[repr(C)]
pub struct IFeedProviderInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EnabledFeedDefinitionIds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedProviderMessage,
    IFeedProviderMessage_Vtbl,
    0x60c2442a_4c9d_4880_ba26_caca9e567dd4
);
impl windows_core::RuntimeType for IFeedProviderMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedProviderMessage",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedProviderMessage,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedProviderMessage {
    pub fn OnMessageReceived<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedMessageReceivedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnMessageReceived)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFeedProviderMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnMessageReceived: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceProvider,
    IFeedResourceProvider_Vtbl,
    0xe1b6266d_88a0_416c_9440_e341cb047cd3
);
impl windows_core::RuntimeType for IFeedResourceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IFeedResourceProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFeedResourceProvider {
    pub fn OnResourceRequested<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FeedResourceRequestedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnResourceRequested)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IFeedResourceProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceProvider";
}
pub trait IFeedResourceProvider_Impl: windows_core::IUnknownImpl {
    fn OnResourceRequested(
        &self,
        args: windows_core::Ref<FeedResourceRequestedArgs>,
    ) -> windows_core::Result<()>;
}
impl IFeedResourceProvider_Vtbl {
    pub const fn new<Identity: IFeedResourceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnResourceRequested<
            Identity: IFeedResourceProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFeedResourceProvider_Impl::OnResourceRequested(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFeedResourceProvider, OFFSET>(
            ),
            OnResourceRequested: OnResourceRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedResourceProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IFeedResourceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnResourceRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceRequest,
    IFeedResourceRequest_Vtbl,
    0xe62e479c_e21f_5863_b4c9_df1be2227981
);
impl windows_core::RuntimeType for IFeedResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequest",
    );
}
#[repr(C)]
pub struct IFeedResourceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Method: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceRequestedArgs,
    IFeedResourceRequestedArgs_Vtbl,
    0x360eb709_0bc9_52c1_9c70_3c7d413173d8
);
impl windows_core::RuntimeType for IFeedResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceRequestedArgs",
    );
}
#[repr(C)]
pub struct IFeedResourceRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedProviderDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FeedDefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceResponse,
    IFeedResourceResponse_Vtbl,
    0xf831824e_7aef_53fc_b7ee_32ade675a3ad
);
impl windows_core::RuntimeType for IFeedResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponse",
    );
}
#[repr(C)]
pub struct IFeedResourceResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Headers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetHeaders: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub StatusCode:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFeedResourceResponseFactory,
    IFeedResourceResponseFactory_Vtbl,
    0xdb01690d_2547_5d7a_b625_d1629f443c5c
);
impl windows_core::RuntimeType for IFeedResourceResponseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Feeds.Providers.IFeedResourceResponseFactory",
    );
}
#[repr(C)]
pub struct IFeedResourceResponseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFileRandomAccessStreamStatics,
    IFileRandomAccessStreamStatics_Vtbl,
    0x73550107_3b57_4b5d_8345_554d2fc621f0
);
impl windows_core::RuntimeType for IFileRandomAccessStreamStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Storage.Streams.IFileRandomAccessStreamStatics",
    );
}
#[repr(C)]
pub struct IFileRandomAccessStreamStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OpenAsync: usize,
    OpenWithOptionsAsync: usize,
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputStream,
    IInputStream_Vtbl,
    0x905a0fe2_bc53_11df_8c49_001e4fc686da
);
impl windows_core::RuntimeType for IInputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IInputStream");
}
windows_core::imp::interface_hierarchy!(
    IInputStream,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IInputStream, IClosable);
impl IInputStream {
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadAsync)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IInputStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        InputStreamOptions,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputStreamReference,
    IInputStreamReference_Vtbl,
    0x43929d18_5ec9_4b5a_919c_4205b0c804b6
);
impl windows_core::RuntimeType for IInputStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Storage.Streams.IInputStreamReference",
    );
}
windows_core::imp::interface_hierarchy!(
    IInputStreamReference,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IInputStreamReference {
    pub fn OpenSequentialReadAsync(
        &self,
    ) -> windows_core::Result<windows_future::IAsyncOperation<IInputStream>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenSequentialReadAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IInputStreamReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenSequentialReadAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOutputStream,
    IOutputStream_Vtbl,
    0x905a0fe6_bc53_11df_8c49_001e4fc686da
);
impl windows_core::RuntimeType for IOutputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IOutputStream");
}
windows_core::imp::interface_hierarchy!(
    IOutputStream,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IOutputStream, IClosable);
impl IOutputStream {
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteAsync)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IOutputStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WriteAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FlushAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPropertySetSerializer,
    IPropertySetSerializer_Vtbl,
    0x6e8ebf1c_ef3d_4376_b20e_5be638aeac77
);
impl windows_core::RuntimeType for IPropertySetSerializer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Storage.Streams.IPropertySetSerializer",
    );
}
windows_core::imp::interface_hierarchy!(
    IPropertySetSerializer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
pub struct IPropertySetSerializer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IRandomAccessStream,
    IRandomAccessStream_Vtbl,
    0x905a0fe1_bc53_11df_8c49_001e4fc686da
);
impl windows_core::RuntimeType for IRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IRandomAccessStream");
}
windows_core::imp::interface_hierarchy!(
    IRandomAccessStream,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IRandomAccessStream, IClosable, IInputStream, IOutputStream);
impl IRandomAccessStream {
    pub fn Size(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Seek)(
                windows_core::Interface::as_raw(self),
                position,
            )
            .ok()
        }
    }
    pub fn CloneStream(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRead)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanWrite)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IRandomAccessStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetInputStreamAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetOutputStreamAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub CloneStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CanRead:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanWrite:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRandomAccessStreamReference,
    IRandomAccessStreamReference_Vtbl,
    0x33ee3134_1dd6_4e3a_8067_d1c162e8642b
);
impl windows_core::RuntimeType for IRandomAccessStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Storage.Streams.IRandomAccessStreamReference",
    );
}
windows_core::imp::interface_hierarchy!(
    IRandomAccessStreamReference,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IRandomAccessStreamReference {
    pub fn OpenReadAsync(
        &self,
    ) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>>
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenReadAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IRandomAccessStreamReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenReadAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRandomAccessStreamReferenceStatics,
    IRandomAccessStreamReferenceStatics_Vtbl,
    0x857309dc_3fbf_4e7d_986f_ef3b1a07a964
);
impl windows_core::RuntimeType for IRandomAccessStreamReferenceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Storage.Streams.IRandomAccessStreamReferenceStatics",
    );
}
#[repr(C)]
pub struct IRandomAccessStreamReferenceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateFromFile: usize,
    pub CreateFromUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFromStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRandomAccessStreamStatics,
    IRandomAccessStreamStatics_Vtbl,
    0x524cedcf_6e29_4ce5_9573_6b753db66c3a
);
impl windows_core::RuntimeType for IRandomAccessStreamStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Storage.Streams.IRandomAccessStreamStatics",
    );
}
#[repr(C)]
pub struct IRandomAccessStreamStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CopyAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CopySizeAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CopyAndCloseAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRandomAccessStreamWithContentType,
    IRandomAccessStreamWithContentType_Vtbl,
    0xcc254827_4b3d_438f_9232_10c76bc7e038
);
impl windows_core::RuntimeType for IRandomAccessStreamWithContentType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Storage.Streams.IRandomAccessStreamWithContentType",
    );
}
windows_core::imp::interface_hierarchy!(
    IRandomAccessStreamWithContentType,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    IRandomAccessStreamWithContentType,
    IClosable,
    IContentTypeProvider,
    IInputStream,
    IOutputStream,
    IRandomAccessStream
);
impl IRandomAccessStreamWithContentType {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(
                windows_core::Interface::as_raw(this),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(
                windows_core::Interface::as_raw(this),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Seek)(
                windows_core::Interface::as_raw(this),
                position,
            )
            .ok()
        }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IRandomAccessStreamWithContentType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IScheduledToastNotification,
    IScheduledToastNotification_Vtbl,
    0x79f577f8_0de7_48cd_9740_9b370490c838
);
impl windows_core::RuntimeType for IScheduledToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IScheduledToastNotification",
    );
}
#[repr(C)]
pub struct IScheduledToastNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeliveryTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
    pub SnoozeInterval: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MaximumSnoozeCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScheduledToastNotification2,
    IScheduledToastNotification2_Vtbl,
    0xa66ea09c_31b4_43b0_b5dd_7a40e85363b1
);
impl windows_core::RuntimeType for IScheduledToastNotification2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IScheduledToastNotification2",
    );
}
#[repr(C)]
pub struct IScheduledToastNotification2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSuppressPopup:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SuppressPopup:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScheduledToastNotification3,
    IScheduledToastNotification3_Vtbl,
    0x98429e8b_bd32_4a3b_9d15_22aea49462a1
);
impl windows_core::RuntimeType for IScheduledToastNotification3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IScheduledToastNotification3",
    );
}
#[repr(C)]
pub struct IScheduledToastNotification3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    NotificationMirroring: usize,
    SetNotificationMirroring: usize,
    pub RemoteId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScheduledToastNotification4,
    IScheduledToastNotification4_Vtbl,
    0x1d4761fd_bdef_4e4a_96be_0101369b58d2
);
impl windows_core::RuntimeType for IScheduledToastNotification4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IScheduledToastNotification4",
    );
}
#[repr(C)]
pub struct IScheduledToastNotification4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScheduledToastNotificationFactory,
    IScheduledToastNotificationFactory_Vtbl,
    0xe7bed191_0bb9_4189_8394_31761b476fd7
);
impl windows_core::RuntimeType for IScheduledToastNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IScheduledToastNotificationFactory",
    );
}
#[repr(C)]
pub struct IScheduledToastNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateScheduledToastNotification: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_time::DateTime,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateScheduledToastNotificationRecurring:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            windows_time::DateTime,
            windows_time::TimeSpan,
            u32,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStorageStreamTransaction,
    IStorageStreamTransaction_Vtbl,
    0xf67cf363_a53d_4d94_ae2c_67232d93acdd
);
impl windows_core::RuntimeType for IStorageStreamTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageStreamTransaction");
}
#[repr(C)]
pub struct IStorageStreamTransaction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Stream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CommitAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStringable,
    IStringable_Vtbl,
    0x96369f54_8eb6_48f0_abce_c1b211e627c3
);
impl windows_core::RuntimeType for IStringable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IStringable");
}
windows_core::imp::interface_hierarchy!(
    IStringable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IStringable {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToString)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
pub struct IStringable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotification,
    IToastNotification_Vtbl,
    0x997e2675_059e_4e60_8b06_1760917c8b80
);
impl windows_core::RuntimeType for IToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotification");
}
#[repr(C)]
pub struct IToastNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Dismissed: usize,
    pub RemoveDismissed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Activated: usize,
    pub RemoveActivated:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Failed: usize,
    pub RemoveFailed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotification2,
    IToastNotification2_Vtbl,
    0x9dfb9fd1_143a_490e_90bf_b9fba7132de7
);
impl windows_core::RuntimeType for IToastNotification2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotification2");
}
#[repr(C)]
pub struct IToastNotification2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSuppressPopup:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SuppressPopup:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotification3,
    IToastNotification3_Vtbl,
    0x31e8aed8_8141_4f99_bc0a_c4ed21297d77
);
impl windows_core::RuntimeType for IToastNotification3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotification3");
}
#[repr(C)]
pub struct IToastNotification3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    NotificationMirroring: usize,
    SetNotificationMirroring: usize,
    pub RemoteId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotification4,
    IToastNotification4_Vtbl,
    0x15154935_28ea_4727_88e9_c58680e2d118
);
impl windows_core::RuntimeType for IToastNotification4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotification4");
}
#[repr(C)]
pub struct IToastNotification4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToastNotification6,
    IToastNotification6_Vtbl,
    0x43ebfe53_89ae_5c1e_a279_3aecfe9b6f54
);
impl windows_core::RuntimeType for IToastNotification6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotification6");
}
#[repr(C)]
pub struct IToastNotification6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpiresOnReboot:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetExpiresOnReboot:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotificationFactory,
    IToastNotificationFactory_Vtbl,
    0x04124b20_82c6_4229_b109_fd9ed4662b53
);
impl windows_core::RuntimeType for IToastNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IToastNotificationFactory",
    );
}
#[repr(C)]
pub struct IToastNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateToastNotification: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotificationManagerStatics,
    IToastNotificationManagerStatics_Vtbl,
    0x50ac103f_d235_4598_bbef_98fe4d1a3ad4
);
impl windows_core::RuntimeType for IToastNotificationManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IToastNotificationManagerStatics",
    );
}
#[repr(C)]
pub struct IToastNotificationManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateToastNotifier: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotificationManagerStatics2,
    IToastNotificationManagerStatics2_Vtbl,
    0x7ab93c52_0e48_4750_ba9d_1a4113981847
);
impl windows_core::RuntimeType for IToastNotificationManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IToastNotificationManagerStatics2",
    );
}
#[repr(C)]
pub struct IToastNotificationManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToastNotificationManagerStatics4,
    IToastNotificationManagerStatics4_Vtbl,
    0x8f993fd3_e516_45fb_8130_398e93fa52c3
);
impl windows_core::RuntimeType for IToastNotificationManagerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IToastNotificationManagerStatics4",
    );
}
#[repr(C)]
pub struct IToastNotificationManagerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToastNotificationManagerStatics5,
    IToastNotificationManagerStatics5_Vtbl,
    0xd6f5f569_d40d_407c_8989_88cab42cfd14
);
impl windows_core::RuntimeType for IToastNotificationManagerStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.UI.Notifications.IToastNotificationManagerStatics5",
    );
}
#[repr(C)]
pub struct IToastNotificationManagerStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToastNotifier,
    IToastNotifier_Vtbl,
    0x75927b93_03f3_41ec_91d3_6e5bac1b38e7
);
impl windows_core::RuntimeType for IToastNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotifier");
}
#[repr(C)]
pub struct IToastNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Show: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Setting: usize,
    pub AddToSchedule: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetScheduledToastNotifications: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToastNotifier2,
    IToastNotifier2_Vtbl,
    0x354389c6_7c01_4bd5_9c20_604340cd2b74
);
impl windows_core::RuntimeType for IToastNotifier2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotifier2");
}
#[repr(C)]
pub struct IToastNotifier2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToastNotifier3,
    IToastNotifier3_Vtbl,
    0xae75a04a_3b0c_51ad_b7e8_b08ab6052549
);
impl windows_core::RuntimeType for IToastNotifier3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Notifications.IToastNotifier3");
}
#[repr(C)]
pub struct IToastNotifier3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ScheduledToastNotificationShowing: usize,
    pub RemoveScheduledToastNotificationShowing:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriEscapeStatics,
    IUriEscapeStatics_Vtbl,
    0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1
);
impl windows_core::RuntimeType for IUriEscapeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriEscapeStatics");
}
#[repr(C)]
pub struct IUriEscapeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnescapeComponent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EscapeComponent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriRuntimeClass,
    IUriRuntimeClass_Vtbl,
    0x9e365e57_48b2_4160_956f_c7385120bbfc
);
impl windows_core::RuntimeType for IUriRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClass");
}
#[repr(C)]
pub struct IUriRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DisplayUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Fragment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Host: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    QueryParsed: usize,
    pub RawUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SchemeName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Suspicious:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub CombineUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriRuntimeClassFactory,
    IUriRuntimeClassFactory_Vtbl,
    0x44a9796f_723e_4fdf_a218_033e75b0c084
);
impl windows_core::RuntimeType for IUriRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClassFactory");
}
#[repr(C)]
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateWithRelativeUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriRuntimeClassWithAbsoluteCanonicalUri,
    IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl,
    0x758d9661_221c_480f_a339_50656673f46f
);
impl windows_core::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri",
    );
}
#[repr(C)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteCanonicalUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DisplayIri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetActionInvokedArgs,
    IWidgetActionInvokedArgs_Vtbl,
    0xc593cc57_04b9_52ca_88ad_46fea21ea340
);
impl windows_core::RuntimeType for IWidgetActionInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetActionInvokedArgs",
    );
}
#[repr(C)]
pub struct IWidgetActionInvokedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Verb: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetAnalyticsInfoReportedArgs,
    IWidgetAnalyticsInfoReportedArgs_Vtbl,
    0x1d9e5fb5_2bce_5350_87b1_d63199526639
);
impl windows_core::RuntimeType for IWidgetAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetAnalyticsInfoReportedArgs",
    );
}
#[repr(C)]
pub struct IWidgetAnalyticsInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AnalyticsJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetContext,
    IWidgetContext_Vtbl,
    0x903c518b_40bc_5bc6_88f7_af9d81c0cdc1
);
impl windows_core::RuntimeType for IWidgetContext {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetContext",
    );
}
#[repr(C)]
pub struct IWidgetContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DefinitionId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Size:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut WidgetSize) -> windows_core::HRESULT,
    pub IsActive:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetContextChangedArgs,
    IWidgetContextChangedArgs_Vtbl,
    0x2c226d54_2252_576b_a197_370b28d25c2f
);
impl windows_core::RuntimeType for IWidgetContextChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetContextChangedArgs",
    );
}
#[repr(C)]
pub struct IWidgetContextChangedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetCustomizationRequestedArgs,
    IWidgetCustomizationRequestedArgs_Vtbl,
    0x41dea311_dd9b_5b8b_b493_3a30552116b8
);
impl windows_core::RuntimeType for IWidgetCustomizationRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetCustomizationRequestedArgs",
    );
}
#[repr(C)]
pub struct IWidgetCustomizationRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetErrorInfoReportedArgs,
    IWidgetErrorInfoReportedArgs_Vtbl,
    0x30efa627_b21f_55d5_b91a_b23b4aa13645
);
impl windows_core::RuntimeType for IWidgetErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetErrorInfoReportedArgs",
    );
}
#[repr(C)]
pub struct IWidgetErrorInfoReportedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ErrorJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetInfo,
    IWidgetInfo_Vtbl,
    0xcea11f42_a020_5db5_89e2_b7dece4ae5cb
);
impl windows_core::RuntimeType for IWidgetInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetInfo",
    );
}
#[repr(C)]
pub struct IWidgetInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LastUpdateTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetInfo2,
    IWidgetInfo2_Vtbl,
    0x081b0a6f_d784_5408_bb29_252fef2926d4
);
impl windows_core::RuntimeType for IWidgetInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetInfo2",
    );
}
#[repr(C)]
pub struct IWidgetInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPlaceholderContent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetManager,
    IWidgetManager_Vtbl,
    0x71cb10c0_671e_48e3_b995_207940397123
);
impl windows_core::RuntimeType for IWidgetManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetManager",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetManager {
    pub fn UpdateWidget<P0>(&self, widgetupdaterequestoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetUpdateRequestOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateWidget)(
                windows_core::Interface::as_raw(self),
                widgetupdaterequestoptions.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetWidgetIds(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetIds)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(
        &self,
        widgetid: &windows_core::HSTRING,
    ) -> windows_core::Result<WidgetInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfo)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetWidgetInfos(&self) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<WidgetInfo>::set_abi_len(core::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteWidget)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWidgetManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWidgetIds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWidgetInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWidgetInfos: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetManager2,
    IWidgetManager2_Vtbl,
    0x55c65a27_8845_406c_9ee1_1e79f0556bef
);
impl windows_core::RuntimeType for IWidgetManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetManager2",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetManager2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetManager2 {
    pub fn SendMessageToContent(
        &self,
        widgetid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SendMessageToContent)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWidgetManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessageToContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetManagerStatics,
    IWidgetManagerStatics_Vtbl,
    0x7f233b06_28e5_5e2b_8c04_a4fa747c28c7
);
impl windows_core::RuntimeType for IWidgetManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetManagerStatics",
    );
}
#[repr(C)]
pub struct IWidgetManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetMessageReceivedArgs,
    IWidgetMessageReceivedArgs_Vtbl,
    0x2261cb2b_c741_5f96_9adb_fb3a7667bcb6
);
impl windows_core::RuntimeType for IWidgetMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetMessageReceivedArgs",
    );
}
#[repr(C)]
pub struct IWidgetMessageReceivedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProvider,
    IWidgetProvider_Vtbl,
    0x5c5774cc_72a0_452d_b9ed_075c0dd25eed
);
impl windows_core::RuntimeType for IWidgetProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProvider {
    pub fn CreateWidget<P0>(&self, widgetcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetContext>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateWidget)(
                windows_core::Interface::as_raw(self),
                widgetcontext.param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteWidget(
        &self,
        widgetid: &windows_core::HSTRING,
        customstate: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteWidget)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                core::mem::transmute_copy(customstate),
            )
            .ok()
        }
    }
    pub fn OnActionInvoked<P0>(&self, actioninvokedargs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetActionInvokedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnActionInvoked)(
                windows_core::Interface::as_raw(self),
                actioninvokedargs.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnWidgetContextChanged<P0>(&self, contextchangedargs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetContextChangedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnWidgetContextChanged)(
                windows_core::Interface::as_raw(self),
                contextchangedargs.param().abi(),
            )
            .ok()
        }
    }
    pub fn Activate<P0>(&self, widgetcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetContext>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Activate)(
                windows_core::Interface::as_raw(self),
                widgetcontext.param().abi(),
            )
            .ok()
        }
    }
    pub fn Deactivate(&self, widgetid: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Deactivate)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProvider";
}
pub trait IWidgetProvider_Impl: windows_core::IUnknownImpl {
    fn CreateWidget(
        &self,
        widgetContext: windows_core::Ref<WidgetContext>,
    ) -> windows_core::Result<()>;
    fn DeleteWidget(
        &self,
        widgetId: &windows_core::HSTRING,
        customState: &windows_core::HSTRING,
    ) -> windows_core::Result<()>;
    fn OnActionInvoked(
        &self,
        actionInvokedArgs: windows_core::Ref<WidgetActionInvokedArgs>,
    ) -> windows_core::Result<()>;
    fn OnWidgetContextChanged(
        &self,
        contextChangedArgs: windows_core::Ref<WidgetContextChangedArgs>,
    ) -> windows_core::Result<()>;
    fn Activate(&self, widgetContext: windows_core::Ref<WidgetContext>)
    -> windows_core::Result<()>;
    fn Deactivate(&self, widgetId: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IWidgetProvider_Vtbl {
    pub const fn new<Identity: IWidgetProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateWidget<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetcontext: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::CreateWidget(this, core::mem::transmute_copy(&widgetcontext))
                    .into()
            }
        }
        unsafe extern "system" fn DeleteWidget<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
            customstate: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::DeleteWidget(
                    this,
                    core::mem::transmute(&widgetid),
                    core::mem::transmute(&customstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnActionInvoked<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            actioninvokedargs: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::OnActionInvoked(
                    this,
                    core::mem::transmute_copy(&actioninvokedargs),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnWidgetContextChanged<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            contextchangedargs: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::OnWidgetContextChanged(
                    this,
                    core::mem::transmute_copy(&contextchangedargs),
                )
                .into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IWidgetProvider_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            widgetcontext: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::Activate(this, core::mem::transmute_copy(&widgetcontext))
                    .into()
            }
        }
        unsafe extern "system" fn Deactivate<
            Identity: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            widgetid: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider_Impl::Deactivate(this, core::mem::transmute(&widgetid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProvider, OFFSET>(),
            CreateWidget: CreateWidget::<Identity, OFFSET>,
            DeleteWidget: DeleteWidget::<Identity, OFFSET>,
            OnActionInvoked: OnActionInvoked::<Identity, OFFSET>,
            OnWidgetContextChanged: OnWidgetContextChanged::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IWidgetProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnActionInvoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnWidgetContextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProvider2,
    IWidgetProvider2_Vtbl,
    0x38c3a963_dd93_479d_9276_04bf84ee1816
);
impl windows_core::RuntimeType for IWidgetProvider2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProvider2",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProvider2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProvider2 {
    pub fn OnCustomizationRequested<P0>(
        &self,
        customizationrequestedargs: P0,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetCustomizationRequestedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnCustomizationRequested)(
                windows_core::Interface::as_raw(self),
                customizationrequestedargs.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IWidgetProvider2 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProvider2";
}
pub trait IWidgetProvider2_Impl: windows_core::IUnknownImpl {
    fn OnCustomizationRequested(
        &self,
        customizationRequestedArgs: windows_core::Ref<WidgetCustomizationRequestedArgs>,
    ) -> windows_core::Result<()>;
}
impl IWidgetProvider2_Vtbl {
    pub const fn new<Identity: IWidgetProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCustomizationRequested<
            Identity: IWidgetProvider2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            customizationrequestedargs: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWidgetProvider2_Impl::OnCustomizationRequested(
                    this,
                    core::mem::transmute_copy(&customizationrequestedargs),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProvider2, OFFSET>(),
            OnCustomizationRequested: OnCustomizationRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetProvider2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IWidgetProvider2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnCustomizationRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProviderAnalytics,
    IWidgetProviderAnalytics_Vtbl,
    0x661985a5_d187_482d_9eef_6fda05d21845
);
impl windows_core::RuntimeType for IWidgetProviderAnalytics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProviderAnalytics",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProviderAnalytics,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProviderAnalytics {
    pub fn OnAnalyticsInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetAnalyticsInfoReportedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnAnalyticsInfoReported)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWidgetProviderAnalytics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnAnalyticsInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProviderErrors,
    IWidgetProviderErrors_Vtbl,
    0x90c1b5f0_0d3a_4ac6_abb7_c97b367b8fcc
);
impl windows_core::RuntimeType for IWidgetProviderErrors {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProviderErrors",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProviderErrors,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProviderErrors {
    pub fn OnErrorInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetErrorInfoReportedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnErrorInfoReported)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWidgetProviderErrors_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnErrorInfoReported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetProviderMessage,
    IWidgetProviderMessage_Vtbl,
    0xea4dc186_9e24_4b35_a5ef_a9f5df72d6ac
);
impl windows_core::RuntimeType for IWidgetProviderMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetProviderMessage",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetProviderMessage,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetProviderMessage {
    pub fn OnMessageReceived<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetMessageReceivedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnMessageReceived)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWidgetProviderMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnMessageReceived: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceProvider,
    IWidgetResourceProvider_Vtbl,
    0xdcf328c0_012c_40f5_bb28_3a1c714d027d
);
impl windows_core::RuntimeType for IWidgetResourceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IWidgetResourceProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetResourceProvider {
    pub fn OnResourceRequested<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetResourceRequestedArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnResourceRequested)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWidgetResourceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnResourceRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceRequest,
    IWidgetResourceRequest_Vtbl,
    0x113d249f_82d9_57cb_8cea_9a5291f2fe22
);
impl windows_core::RuntimeType for IWidgetResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequest",
    );
}
#[repr(C)]
pub struct IWidgetResourceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Method: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Headers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceRequestedArgs,
    IWidgetResourceRequestedArgs_Vtbl,
    0x2bb30f4d_0166_58e3_aaf6_31b2ae970bcd
);
impl windows_core::RuntimeType for IWidgetResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequestedArgs",
    );
}
#[repr(C)]
pub struct IWidgetResourceRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceResponse,
    IWidgetResourceResponse_Vtbl,
    0x03a2d32c_2e9e_54a3_b084_1479d5060f80
);
impl windows_core::RuntimeType for IWidgetResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponse",
    );
}
#[repr(C)]
pub struct IWidgetResourceResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Headers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub StatusCode:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetResourceResponseFactory,
    IWidgetResourceResponseFactory_Vtbl,
    0x08881ef1_a78a_5804_b070_9153a8657f85
);
impl windows_core::RuntimeType for IWidgetResourceResponseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponseFactory",
    );
}
#[repr(C)]
pub struct IWidgetResourceResponseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptions,
    IWidgetUpdateRequestOptions_Vtbl,
    0xb09ca8f7_7424_5687_baaf_7dd6fa639672
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions",
    );
}
#[repr(C)]
pub struct IWidgetUpdateRequestOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WidgetId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCustomState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptions2,
    IWidgetUpdateRequestOptions2_Vtbl,
    0x77c4efc4_38f3_57a5_aba1_f83f257b899e
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions2",
    );
}
#[repr(C)]
pub struct IWidgetUpdateRequestOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPlaceholderContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetIsPlaceholderContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptionsFactory,
    IWidgetUpdateRequestOptionsFactory_Vtbl,
    0xe0e00af8_1d10_57a8_9419_3f568e854daa
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsFactory",
    );
}
#[repr(C)]
pub struct IWidgetUpdateRequestOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidgetUpdateRequestOptionsStatics,
    IWidgetUpdateRequestOptionsStatics_Vtbl,
    0x4645b5e3_d332_5d11_82f0_3607e5df6018
);
impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsStatics",
    );
}
#[repr(C)]
pub struct IWidgetUpdateRequestOptionsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnsetValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXmlDocument,
    IXmlDocument_Vtbl,
    0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494
);
impl windows_core::RuntimeType for IXmlDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Data.Xml.Dom.IXmlDocument");
}
#[repr(C)]
pub struct IXmlDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Doctype: usize,
    Implementation: usize,
    DocumentElement: usize,
    CreateElement: usize,
    CreateDocumentFragment: usize,
    CreateTextNode: usize,
    CreateComment: usize,
    CreateProcessingInstruction: usize,
    CreateAttribute: usize,
    CreateEntityReference: usize,
    GetElementsByTagName: usize,
    CreateCDataSection: usize,
    pub DocumentUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateAttributeNS: usize,
    CreateElementNS: usize,
    GetElementById: usize,
    pub ImportNode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXmlDocumentIO,
    IXmlDocumentIO_Vtbl,
    0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637
);
impl windows_core::RuntimeType for IXmlDocumentIO {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Data.Xml.Dom.IXmlDocumentIO");
}
#[repr(C)]
pub struct IXmlDocumentIO_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LoadXml: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXmlDocumentIO2,
    IXmlDocumentIO2_Vtbl,
    0x5d034661_7bd8_4ad5_9ebf_81e6347263b1
);
impl windows_core::RuntimeType for IXmlDocumentIO2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Data.Xml.Dom.IXmlDocumentIO2");
}
#[repr(C)]
pub struct IXmlDocumentIO2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LoadXmlFromBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXmlDocumentStatics,
    IXmlDocumentStatics_Vtbl,
    0x5543d254_d757_4b79_9539_232b18f50bf1
);
impl windows_core::RuntimeType for IXmlDocumentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Data.Xml.Dom.IXmlDocumentStatics");
}
#[repr(C)]
pub struct IXmlDocumentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LoadFromUriAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXmlNode,
    IXmlNode_Vtbl,
    0x1c741d59_2122_47d5_a856_83f3d4214875
);
impl windows_core::RuntimeType for IXmlNode {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Data.Xml.Dom.IXmlNode");
}
windows_core::imp::interface_hierarchy!(
    IXmlNode,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl IXmlNode {
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NodeValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetNodeValue)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NodeName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentNode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirstChild)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastChild)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreviousSibling)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextSibling)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasChildNodes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OwnerDocument)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(
        &self,
        newchild: P0,
        referencechild: P1,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertBefore)(
                windows_core::Interface::as_raw(self),
                newchild.param().abi(),
                referencechild.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(
        &self,
        newchild: P0,
        referencechild: P1,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReplaceChild)(
                windows_core::Interface::as_raw(self),
                newchild.param().abi(),
                referencechild.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoveChild)(
                windows_core::Interface::as_raw(self),
                childnode.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppendChild)(
                windows_core::Interface::as_raw(self),
                newchild.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneNode)(
                windows_core::Interface::as_raw(self),
                deep,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamespaceUri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Prefix)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Normalize)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrefix)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<Self> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(xpath),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P1>(
        &self,
        xpath: &windows_core::HSTRING,
        namespaces: P1,
    ) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(xpath),
                namespaces.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetInnerText)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IXmlNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NodeValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    NodeType: usize,
    pub NodeName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ParentNode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ChildNodes: usize,
    pub FirstChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LastChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PreviousSibling: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NextSibling: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Attributes: usize,
    pub HasChildNodes:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub OwnerDocument: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InsertBefore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CloneNode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        bool,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LocalName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Prefix: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Normalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrefix: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXmlNodeSelector,
    IXmlNodeSelector_Vtbl,
    0x63dbba8b_d0db_4fe1_b745_f9433afdc25b
);
impl windows_core::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Data.Xml.Dom.IXmlNodeSelector");
}
windows_core::imp::interface_hierarchy!(
    IXmlNodeSelector,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IXmlNodeSelector {
    pub fn SelectSingleNode(
        &self,
        xpath: &windows_core::HSTRING,
    ) -> windows_core::Result<IXmlNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectSingleNode)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(xpath),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P1>(
        &self,
        xpath: &windows_core::HSTRING,
        namespaces: P1,
    ) -> windows_core::Result<IXmlNode>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectSingleNodeNS)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(xpath),
                namespaces.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IXmlNodeSelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectSingleNode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SelectNodes: usize,
    pub SelectSingleNodeNS: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXmlNodeSerializer,
    IXmlNodeSerializer_Vtbl,
    0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c
);
impl windows_core::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Data.Xml.Dom.IXmlNodeSerializer");
}
windows_core::imp::interface_hierarchy!(
    IXmlNodeSerializer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IXmlNodeSerializer {
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXml)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InnerText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInnerText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IXmlNodeSerializer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetXml: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InnerText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InMemoryRandomAccessStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InMemoryRandomAccessStream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IRandomAccessStream
);
windows_core::imp::required_hierarchy!(
    InMemoryRandomAccessStream,
    IClosable,
    IInputStream,
    IOutputStream
);
impl InMemoryRandomAccessStream {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            InMemoryRandomAccessStream,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Seek)(
                windows_core::Interface::as_raw(self),
                position,
            )
            .ok()
        }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRead)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanWrite)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InMemoryRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStream>();
}
unsafe impl windows_core::Interface for InMemoryRandomAccessStream {
    type Vtable = <IRandomAccessStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InMemoryRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.InMemoryRandomAccessStream";
}
unsafe impl Send for InMemoryRandomAccessStream {}
unsafe impl Sync for InMemoryRandomAccessStream {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: Self = Self(0);
    pub const Partial: Self = Self(1);
    pub const ReadAhead: Self = Self(2);
}
impl windows_core::imp::TypeKind for InputStreamOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputStreamOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Storage.Streams.InputStreamOptions;u4)",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.InputStreamOptions");
}
impl InputStreamOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InputStreamOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InputStreamOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InputStreamOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for InputStreamOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for InputStreamOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputStreamOverStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputStreamOverStream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IInputStream
);
windows_core::imp::required_hierarchy!(InputStreamOverStream, IClosable);
impl InputStreamOverStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadAsync)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InputStreamOverStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputStream>();
}
unsafe impl windows_core::Interface for InputStreamOverStream {
    type Vtable = <IInputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.InputStreamOverStream";
}
unsafe impl Send for InputStreamOverStream {}
unsafe impl Sync for InputStreamOverStream {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputStreamOverStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    OutputStreamOverStream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IOutputStream
);
windows_core::imp::required_hierarchy!(OutputStreamOverStream, IClosable);
impl OutputStreamOverStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteAsync)(
                windows_core::Interface::as_raw(self),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OutputStreamOverStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IOutputStream>();
}
unsafe impl windows_core::Interface for OutputStreamOverStream {
    type Vtable = <IOutputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOutputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OutputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.OutputStreamOverStream";
}
unsafe impl Send for OutputStreamOverStream {}
unsafe impl Sync for OutputStreamOverStream {}
pub struct RandomAccessStream;
impl RandomAccessStream {
    pub fn CopyAsync<P0, P1>(
        source: P0,
        destination: P1,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: windows_core::Param<IInputStream>,
        P1: windows_core::Param<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyAsync)(
                windows_core::Interface::as_raw(this),
                source.param().abi(),
                destination.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CopySizeAsync<P0, P1>(
        source: P0,
        destination: P1,
        bytestocopy: u64,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: windows_core::Param<IInputStream>,
        P1: windows_core::Param<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopySizeAsync)(
                windows_core::Interface::as_raw(this),
                source.param().abi(),
                destination.param().abi(),
                bytestocopy,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CopyAndCloseAsync<P0, P1>(
        source: P0,
        destination: P1,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: windows_core::Param<IInputStream>,
        P1: windows_core::Param<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyAndCloseAsync)(
                windows_core::Interface::as_raw(this),
                source.param().abi(),
                destination.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IRandomAccessStreamStatics<
        R,
        F: FnOnce(&IRandomAccessStreamStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            RandomAccessStream,
            IRandomAccessStreamStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for RandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStream";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomAccessStreamOverStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RandomAccessStreamOverStream,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IRandomAccessStream
);
windows_core::imp::required_hierarchy!(
    RandomAccessStreamOverStream,
    IClosable,
    IInputStream,
    IOutputStream
);
impl RandomAccessStreamOverStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ReadAsync<P0>(
        &self,
        buffer: P0,
        count: u32,
        options: InputStreamOptions,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                count,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(
        &self,
        buffer: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAt)(
                windows_core::Interface::as_raw(self),
                position,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Seek)(
                windows_core::Interface::as_raw(self),
                position,
            )
            .ok()
        }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneStream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRead)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanWrite)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RandomAccessStreamOverStream {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStream>();
}
unsafe impl windows_core::Interface for RandomAccessStreamOverStream {
    type Vtable = <IRandomAccessStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RandomAccessStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamOverStream";
}
unsafe impl Send for RandomAccessStreamOverStream {}
unsafe impl Sync for RandomAccessStreamOverStream {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomAccessStreamReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RandomAccessStreamReference,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IRandomAccessStreamReference
);
impl RandomAccessStreamReference {
    pub fn OpenReadAsync(
        &self,
    ) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>>
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenReadAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateFromUri<P0>(uri: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Uri>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUri)(
                windows_core::Interface::as_raw(this),
                uri.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromStream<P0>(stream: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IRandomAccessStream>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStream)(
                windows_core::Interface::as_raw(this),
                stream.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IRandomAccessStreamReferenceStatics<
        R,
        F: FnOnce(&IRandomAccessStreamReferenceStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            RandomAccessStreamReference,
            IRandomAccessStreamReferenceStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RandomAccessStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStreamReference>();
}
unsafe impl windows_core::Interface for RandomAccessStreamReference {
    type Vtable = <IRandomAccessStreamReference as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStreamReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
unsafe impl Send for RandomAccessStreamReference {}
unsafe impl Sync for RandomAccessStreamReference {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledToastNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ScheduledToastNotification,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ScheduledToastNotification {
    pub fn Content(&self) -> windows_core::Result<XmlDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DeliveryTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeliveryTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SnoozeInterval(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SnoozeInterval)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::TimeSpan>| r__.Value())
        }
    }
    pub fn MaximumSnoozeCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaximumSnoozeCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetId)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetGroup)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetSuppressPopup)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SuppressPopup(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressPopup)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRemoteId)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<windows_time::DateTime> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::DateTime>| r__.Value())
        }
    }
    pub fn SetExpirationTime(
        &self,
        value: Option<windows_time::DateTime>,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification4>(self)?;
        let value__ =
            value.map(<windows_reference::IReference<windows_time::DateTime> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(this).SetExpirationTime)(
                windows_core::Interface::as_raw(this),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn CreateScheduledToastNotification<P0>(
        content: P0,
        deliverytime: windows_time::DateTime,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<XmlDocument>,
    {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateScheduledToastNotification)(
                windows_core::Interface::as_raw(this),
                content.param().abi(),
                deliverytime,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateScheduledToastNotificationRecurring<P0>(
        content: P0,
        deliverytime: windows_time::DateTime,
        snoozeinterval: windows_time::TimeSpan,
        maximumsnoozecount: u32,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<XmlDocument>,
    {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateScheduledToastNotificationRecurring)(
                windows_core::Interface::as_raw(this),
                content.param().abi(),
                deliverytime,
                snoozeinterval,
                maximumsnoozecount,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IScheduledToastNotificationFactory<
        R,
        F: FnOnce(&IScheduledToastNotificationFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ScheduledToastNotification,
            IScheduledToastNotificationFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScheduledToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IScheduledToastNotification>();
}
unsafe impl windows_core::Interface for ScheduledToastNotification {
    type Vtable = <IScheduledToastNotification as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScheduledToastNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotification";
}
unsafe impl Send for ScheduledToastNotification {}
unsafe impl Sync for ScheduledToastNotification {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageStreamTransaction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    StorageStreamTransaction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(StorageStreamTransaction, IClosable);
impl StorageStreamTransaction {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Stream(&self) -> windows_core::Result<IRandomAccessStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stream)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CommitAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageStreamTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStorageStreamTransaction>();
}
unsafe impl windows_core::Interface for StorageStreamTransaction {
    type Vtable = <IStorageStreamTransaction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageStreamTransaction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageStreamTransaction {
    const NAME: &'static str = "Windows.Storage.StorageStreamTransaction";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToastNotification,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ToastNotification {
    pub fn Content(&self) -> windows_core::Result<XmlDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetExpirationTime(
        &self,
        value: Option<windows_time::DateTime>,
    ) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<windows_time::DateTime> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetExpirationTime)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpirationTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::DateTime>| r__.Value())
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetGroup)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetSuppressPopup)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SuppressPopup(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressPopup)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRemoteId)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExpiresOnReboot(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpiresOnReboot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExpiresOnReboot)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CreateToastNotification<P0>(content: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<XmlDocument>,
    {
        Self::IToastNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotification)(
                windows_core::Interface::as_raw(this),
                content.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IToastNotificationFactory<
        R,
        F: FnOnce(&IToastNotificationFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ToastNotification,
            IToastNotificationFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToastNotification>();
}
unsafe impl windows_core::Interface for ToastNotification {
    type Vtable = <IToastNotification as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToastNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotification";
}
unsafe impl Send for ToastNotification {}
unsafe impl Sync for ToastNotification {}
pub struct ToastNotificationManager;
impl ToastNotificationManager {
    pub fn CreateToastNotifier() -> windows_core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifier)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateToastNotifierWithId(
        applicationid: &windows_core::HSTRING,
    ) -> windows_core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifierWithId)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(applicationid),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IToastNotificationManagerStatics<
        R,
        F: FnOnce(&IToastNotificationManagerStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IToastNotificationManagerStatics2<
        R,
        F: FnOnce(&IToastNotificationManagerStatics2) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics2,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IToastNotificationManagerStatics4<
        R,
        F: FnOnce(&IToastNotificationManagerStatics4) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics4,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IToastNotificationManagerStatics5<
        R,
        F: FnOnce(&IToastNotificationManagerStatics5) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ToastNotificationManager,
            IToastNotificationManagerStatics5,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ToastNotificationManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManager";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastNotifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToastNotifier,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ToastNotifier {
    pub fn Show<P0>(&self, notification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ToastNotification>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Show)(
                windows_core::Interface::as_raw(self),
                notification.param().abi(),
            )
            .ok()
        }
    }
    pub fn Hide<P0>(&self, notification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ToastNotification>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Hide)(
                windows_core::Interface::as_raw(self),
                notification.param().abi(),
            )
            .ok()
        }
    }
    pub fn AddToSchedule<P0>(&self, scheduledtoast: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ScheduledToastNotification>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddToSchedule)(
                windows_core::Interface::as_raw(self),
                scheduledtoast.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveFromSchedule<P0>(&self, scheduledtoast: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ScheduledToastNotification>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveFromSchedule)(
                windows_core::Interface::as_raw(self),
                scheduledtoast.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetScheduledToastNotifications(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<ScheduledToastNotification>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScheduledToastNotifications)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToastNotifier>();
}
unsafe impl windows_core::Interface for ToastNotifier {
    type Vtable = <IToastNotifier as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToastNotifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotifier";
}
unsafe impl Send for ToastNotifier {}
unsafe impl Sync for ToastNotifier {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: Self = Self(0);
    pub const Utf16LE: Self = Self(1);
    pub const Utf16BE: Self = Self(2);
}
impl windows_core::imp::TypeKind for UnicodeEncoding {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for UnicodeEncoding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Storage.Streams.UnicodeEncoding;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.UnicodeEncoding");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Uri, IStringable);
impl Uri {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn UnescapeComponent(
        tounescape: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnescapeComponent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(tounescape),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    pub fn EscapeComponent(
        toescape: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EscapeComponent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(toescape),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    pub fn AbsoluteUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AbsoluteUri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayUri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Extension(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Extension)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Fragment(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Fragment)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Host(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Host)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Password(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Password)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Query(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn RawUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RawUri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SchemeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SchemeName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Port(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Port)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Suspicious(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suspicious)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Equals<P0>(&self, puri: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Equals)(
                windows_core::Interface::as_raw(self),
                puri.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CombineUri(&self, relativeuri: &windows_core::HSTRING) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CombineUri)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(relativeuri),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUri)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(uri),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWithRelativeUri(
        baseuri: &windows_core::HSTRING,
        relativeuri: &windows_core::HSTRING,
    ) -> windows_core::Result<Self> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithRelativeUri)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(baseuri),
                core::mem::transmute_copy(relativeuri),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn AbsoluteCanonicalUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this =
            &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteCanonicalUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayIri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this =
            &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayIri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUriRuntimeClassFactory<
        R,
        F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Uri {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
}
unsafe impl windows_core::Interface for Uri {
    type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetActionInvokedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetActionInvokedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetActionInvokedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Verb)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetActionInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetActionInvokedArgs>();
}
unsafe impl windows_core::Interface for WidgetActionInvokedArgs {
    type Vtable = <IWidgetActionInvokedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetActionInvokedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetActionInvokedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetActionInvokedArgs";
}
unsafe impl Send for WidgetActionInvokedArgs {}
unsafe impl Sync for WidgetActionInvokedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetAnalyticsInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetAnalyticsInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetAnalyticsInfoReportedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnalyticsJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetAnalyticsInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetAnalyticsInfoReportedArgs>();
}
unsafe impl windows_core::Interface for WidgetAnalyticsInfoReportedArgs {
    type Vtable = <IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetAnalyticsInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.WidgetAnalyticsInfoReportedArgs";
}
unsafe impl Send for WidgetAnalyticsInfoReportedArgs {}
unsafe impl Sync for WidgetAnalyticsInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetContext,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetContext {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefinitionId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<WidgetSize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsActive)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WidgetContext {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetContext>();
}
unsafe impl windows_core::Interface for WidgetContext {
    type Vtable = <IWidgetContext as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetContext {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContext";
}
unsafe impl Send for WidgetContext {}
unsafe impl Sync for WidgetContext {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetContextChangedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetContextChangedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetContextChangedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetContextChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetContextChangedArgs>();
}
unsafe impl windows_core::Interface for WidgetContextChangedArgs {
    type Vtable = <IWidgetContextChangedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetContextChangedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetContextChangedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContextChangedArgs";
}
unsafe impl Send for WidgetContextChangedArgs {}
unsafe impl Sync for WidgetContextChangedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetCustomizationRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetCustomizationRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetCustomizationRequestedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetCustomizationRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetCustomizationRequestedArgs>();
}
unsafe impl windows_core::Interface for WidgetCustomizationRequestedArgs {
    type Vtable = <IWidgetCustomizationRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IWidgetCustomizationRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetCustomizationRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.WidgetCustomizationRequestedArgs";
}
unsafe impl Send for WidgetCustomizationRequestedArgs {}
unsafe impl Sync for WidgetCustomizationRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetErrorInfoReportedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetErrorInfoReportedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetErrorInfoReportedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetErrorInfoReportedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetErrorInfoReportedArgs>();
}
unsafe impl windows_core::Interface for WidgetErrorInfoReportedArgs {
    type Vtable = <IWidgetErrorInfoReportedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetErrorInfoReportedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetErrorInfoReportedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetErrorInfoReportedArgs";
}
unsafe impl Send for WidgetErrorInfoReportedArgs {}
unsafe impl Sync for WidgetErrorInfoReportedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetInfo {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Template)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn LastUpdateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastUpdateTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsPlaceholderContent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IWidgetInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlaceholderContent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WidgetInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetInfo>();
}
unsafe impl windows_core::Interface for WidgetInfo {
    type Vtable = <IWidgetInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetInfo";
}
unsafe impl Send for WidgetInfo {}
unsafe impl Sync for WidgetInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetManager,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IWidgetManager
);
windows_core::imp::required_hierarchy!(WidgetManager, IWidgetManager2);
impl WidgetManager {
    pub fn UpdateWidget<P0>(&self, widgetupdaterequestoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetUpdateRequestOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateWidget)(
                windows_core::Interface::as_raw(self),
                widgetupdaterequestoptions.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetWidgetIds(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetIds)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(
        &self,
        widgetid: &windows_core::HSTRING,
    ) -> windows_core::Result<WidgetInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfo)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetWidgetInfos(&self) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<WidgetInfo>::set_abi_len(core::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteWidget)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
    pub fn SendMessageToContent(
        &self,
        widgetid: &windows_core::HSTRING,
        message: &windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWidgetManager2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SendMessageToContent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(widgetid),
                core::mem::transmute_copy(message),
            )
            .ok()
        }
    }
    pub fn GetDefault() -> windows_core::Result<Self> {
        Self::IWidgetManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IWidgetManagerStatics<R, F: FnOnce(&IWidgetManagerStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WidgetManager, IWidgetManagerStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WidgetManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetManager>();
}
unsafe impl windows_core::Interface for WidgetManager {
    type Vtable = <IWidgetManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetManager";
}
unsafe impl Send for WidgetManager {}
unsafe impl Sync for WidgetManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetMessageReceivedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetMessageReceivedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetMessageReceivedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetMessageReceivedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetMessageReceivedArgs>();
}
unsafe impl windows_core::Interface for WidgetMessageReceivedArgs {
    type Vtable = <IWidgetMessageReceivedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetMessageReceivedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetMessageReceivedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetMessageReceivedArgs";
}
unsafe impl Send for WidgetMessageReceivedArgs {}
unsafe impl Sync for WidgetMessageReceivedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetResourceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetResourceRequest,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetResourceRequest {
    pub fn Uri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMethod(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMethod)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Content(&self) -> windows_core::Result<IRandomAccessStreamReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRandomAccessStreamReference>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Headers(
        &self,
    ) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Headers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WidgetResourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceRequest>();
}
unsafe impl windows_core::Interface for WidgetResourceRequest {
    type Vtable = <IWidgetResourceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetResourceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetResourceRequest {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetResourceRequest";
}
unsafe impl Send for WidgetResourceRequest {}
unsafe impl Sync for WidgetResourceRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetResourceRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetResourceRequestedArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetResourceRequestedArgs {
    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Request(&self) -> windows_core::Result<WidgetResourceRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Response(&self) -> windows_core::Result<WidgetResourceResponse> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Response)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResponse<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WidgetResourceResponse>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResponse)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for WidgetResourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceRequestedArgs>();
}
unsafe impl windows_core::Interface for WidgetResourceRequestedArgs {
    type Vtable = <IWidgetResourceRequestedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetResourceRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetResourceRequestedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetResourceRequestedArgs";
}
unsafe impl Send for WidgetResourceRequestedArgs {}
unsafe impl Sync for WidgetResourceRequestedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetResourceResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetResourceResponse,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetResourceResponse {
    pub fn Content(&self) -> windows_core::Result<IRandomAccessStreamReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Headers(
        &self,
    ) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Headers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReasonPhrase(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReasonPhrase)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CreateInstance<P0>(
        content: P0,
        reasonphrase: &windows_core::HSTRING,
        statuscode: i32,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IRandomAccessStreamReference>,
    {
        Self::IWidgetResourceResponseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                content.param().abi(),
                core::mem::transmute_copy(reasonphrase),
                statuscode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IWidgetResourceResponseFactory<
        R,
        F: FnOnce(&IWidgetResourceResponseFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WidgetResourceResponse,
            IWidgetResourceResponseFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WidgetResourceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceResponse>();
}
unsafe impl windows_core::Interface for WidgetResourceResponse {
    type Vtable = <IWidgetResourceResponse as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetResourceResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetResourceResponse {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetResourceResponse";
}
unsafe impl Send for WidgetResourceResponse {}
unsafe impl Sync for WidgetResourceResponse {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WidgetSize(pub i32);
impl WidgetSize {
    pub const Small: Self = Self(0);
    pub const Medium: Self = Self(1);
    pub const Large: Self = Self(2);
}
impl windows_core::imp::TypeKind for WidgetSize {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for WidgetSize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.Widgets.WidgetSize;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Microsoft.Windows.Widgets.WidgetSize");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetUpdateRequestOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WidgetUpdateRequestOptions,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WidgetUpdateRequestOptions {
    pub fn WidgetId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidgetId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Template)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTemplate(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTemplate)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetData)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCustomState(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCustomState)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsPlaceholderContent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlaceholderContent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<bool>| r__.Value())
        }
    }
    pub fn SetIsPlaceholderContent(&self, value: Option<bool>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
        let value__ = value.map(<windows_reference::IReference<bool> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(this).SetIsPlaceholderContent)(
                windows_core::Interface::as_raw(this),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn CreateInstance(widgetid: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IWidgetUpdateRequestOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn UnsetValue() -> windows_core::Result<windows_core::HSTRING> {
        Self::IWidgetUpdateRequestOptionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsetValue)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    fn IWidgetUpdateRequestOptionsFactory<
        R,
        F: FnOnce(&IWidgetUpdateRequestOptionsFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WidgetUpdateRequestOptions,
            IWidgetUpdateRequestOptionsFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IWidgetUpdateRequestOptionsStatics<
        R,
        F: FnOnce(&IWidgetUpdateRequestOptionsStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WidgetUpdateRequestOptions,
            IWidgetUpdateRequestOptionsStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WidgetUpdateRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetUpdateRequestOptions>();
}
unsafe impl windows_core::Interface for WidgetUpdateRequestOptions {
    type Vtable = <IWidgetUpdateRequestOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidgetUpdateRequestOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WidgetUpdateRequestOptions {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetUpdateRequestOptions";
}
unsafe impl Send for WidgetUpdateRequestOptions {}
unsafe impl Sync for WidgetUpdateRequestOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlDocument(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XmlDocument,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(XmlDocument, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlDocument {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XmlDocument,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DocumentUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentUri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ImportNode<P0>(&self, node: P0, deep: bool) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportNode)(
                windows_core::Interface::as_raw(self),
                node.param().abi(),
                deep,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn LoadXml(&self, xml: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).LoadXml)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(xml),
            )
            .ok()
        }
    }
    pub fn LoadXmlFromBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).LoadXmlFromBuffer)(
                windows_core::Interface::as_raw(this),
                buffer.param().abi(),
            )
            .ok()
        }
    }
    pub fn LoadFromUriAsync<P0>(
        uri: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
    where
        P0: windows_core::Param<Uri>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromUriAsync)(
                windows_core::Interface::as_raw(this),
                uri.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetNodeValue)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<Self> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(
        &self,
        newchild: P0,
        referencechild: P1,
    ) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(
                windows_core::Interface::as_raw(this),
                newchild.param().abi(),
                referencechild.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(
        &self,
        newchild: P0,
        referencechild: P1,
    ) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(
                windows_core::Interface::as_raw(this),
                newchild.param().abi(),
                referencechild.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(
                windows_core::Interface::as_raw(this),
                childnode.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(
                windows_core::Interface::as_raw(this),
                newchild.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(
                windows_core::Interface::as_raw(this),
                deep,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPrefix)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn SelectSingleNode(
        &self,
        xpath: &windows_core::HSTRING,
    ) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(xpath),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P1>(
        &self,
        xpath: &windows_core::HSTRING,
        namespaces: P1,
    ) -> windows_core::Result<IXmlNode>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(xpath),
                namespaces.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetInnerText)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XmlDocument, IXmlDocumentStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XmlDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXmlDocument>();
}
unsafe impl windows_core::Interface for XmlDocument {
    type Vtable = <IXmlDocument as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXmlDocument as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
unsafe impl Send for XmlDocument {}
unsafe impl Sync for XmlDocument {}
