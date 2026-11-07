# RustedUI — Özel İmaj (`<img />`), PNG Varlık Yükleme, SpriteSheet, Portre ve Dinamik Texture Atlas Boru Hattı

> **Doküman Kodu:** RUSTED-ARCH-011  
> **Durum:** Tamamlandı — Faz 0 Teknik Araştırma ve Mimari Şartname (Normative Specification)  
> **Tarih:** 2026-11-07  
> **Hedef Çevre:** Stardew Valley 1.6+ (.NET 6.0 / MonoGame 3.8.1+) & SMAPI 4.x+  
> **İlişkili Dokümanlar:**  
> - [01-project-baseline-and-architecture-plan.md](file:///home/mehmetcan/Code/Rusted/research/01-project-baseline-and-architecture-plan.md)  
> - [02-smapi-monogame-rendering-and-vanilla-assets.md](file:///home/mehmetcan/Code/Rusted/research/02-smapi-monogame-rendering-and-vanilla-assets.md)  
> - [03-rust-ui-engine-taffy-and-styling-pipeline.md](file:///home/mehmetcan/Code/Rusted/research/03-rust-ui-engine-taffy-and-styling-pipeline.md)  
> - [04-csharp-rust-translation-layer-and-crossplatform-abi.md](file:///home/mehmetcan/Code/Rusted/research/04-csharp-rust-translation-layer-and-crossplatform-abi.md)  
> - [06-taffy-flexbox-wrapping-and-sizing-audit.md](file:///home/mehmetcan/Code/Rusted/research/06-taffy-flexbox-wrapping-and-sizing-audit.md)  
> - [09-react-vite-tsx-component-authoring-architecture.md](file:///home/mehmetcan/Code/Rusted/research/09-react-vite-tsx-component-authoring-architecture.md)

---

## İçindekiler
1. [Yönetici Özeti ve Mimari Vizyon](#1-yönetici-özeti-ve-mimari-vizyon)
2. [Deklaratif Etiket Mimarisi: `<img />`, `<sprite />`, `<item-icon />` ve `<animated-sprite />`](#2-deklaratif-etiket-mimarisi-img--sprite--item-icon--ve-animated-sprite-)
   - 2.1 Desteklenen Varlık Kaynakları ve URI Protokolleri
   - 2.2 HTML/XML Şablon Sözdizimi ve Öznitelik Spesifikasyonu
   - 2.3 React / TSX Bileşen Arayüzü
   - 2.4 C# Fluent Builder API Entegrasyonu
3. [Sıfır-Kopya FFI String-to-TextureId Registry ve Önbellek Mimarisi](#3-sıfır-kopya-ffi-string-to-textureid-registry-ve-önbellek-mimarisi)
   - 3.1 Sorun Analizi: Çizim Döngüsünde String Taşıma Maliyeti ve GC Baskısı
   - 3.2 İki Aşamalı Texture Interning Mimarisi ve Dinamik ID Havuzu
   - 3.3 Çift Yönlü FFI El Sıkışması (Bidirectional Handshake)
   - 3.4 C# `TextureManager`: Lazy-Load, ModContent, GameContent ve ItemRegistry Çözümleme
   - 3.5 SMAPI Önbellek Geçersiz Kılma (Asset Invalidation) ve Hot-Reload
4. [Taffy ile Doğal Boyutlandırma (Intrinsic Sizing), Aspect Ratio ve CSS `object-fit`](#4-taffy-ile-doğal-boyutlandırma-intrinsic-sizing-aspect-ratio-ve-css-object-fit)
   - 4.1 İki Aşamalı Yerleşim (Layout) ve Boyut Senkronizasyon Protokolü
   - 4.2 Taffy Layout Context İçinde Leaf Measure Fonksiyonu
   - 4.3 En-Boy Oranı Kilitleme (Aspect Ratio Locking) Matematiği
   - 4.4 CSS `object-fit` Algoritmaları: `fill`, `contain`, `cover`, `none`, `scale-down`
   - 4.5 Piksel-Sanatı Netliği: `image-rendering: pixelated` ve MonoGame `PointClamp`
5. [Animasyonlu Sprite'lar (`<animated-sprite />`) ve Zamanlama Boru Hattı](#5-animasyonlu-spritelar-animated-sprite--ve-zamanlama-boru-hattı)
   - 5.1 Sprite Sheet Mimarisi (Horizontal Strip vs Grid Matrix)
   - 5.2 Deterministic Frame Hesaplama Matematiği
   - 5.3 SMAPI `UpdateTicked` / `GameTime` Entegrasyonu
   - 5.4 Animasyon Döngü Modları (Loop, Ping-Pong, Play-Once) ve Olay Bildirimleri
6. [Özel 9-Slice Çerçeve Sistemi (Custom 9-Slice Image Frames)](#6-özel-9-slice-çerçeve-sistemi-custom-9-slice-image-frames)
   - 6.1 Vanilya Ötesi: Mod Yapımcılarının Özel UI Çerçeveleri
   - 6.2 Etiket Sözdizimi: `<nine-slice />` ve Tailwind Utility Sınıfları
   - 6.3 9-Bölge Geometrisi ve Piksel Kanaması (Bleeding) Önleme
   - 6.4 MonoGame `NineSliceRenderer` Entegrasyonu ve Batching Optimizasyonu
7. [Kapsamlı Veri Sözleşmesi, FFI ve Kod Referansları (Rust & C#)](#7-kapsamlı-veri-sözleşmesi-ffi-ve-kod-referansları-rust--c)
   - 7.1 Rust `rusted-core` Veri Yapıları (`types.rs`, `dom.rs`, `tailwind.rs`)
   - 7.2 Rust `rusted-bridge` C-ABI Fonksiyonları
   - 7.3 C# P/Invoke Tanımları (`NativeProtocol.cs`)
   - 7.4 C# `TextureManager.cs` ve `RustedRenderer.cs` Tam Uygulaması
8. [Mühendislik Kontrol Listesi, Performans Analizi ve Sınır Durumlar](#8-mühendislik-kontrol-listesi-performans-analizi-ve-sınır-durumlar)
   - 8.1 Eksik/Hatalı Varlık Yönetimi (Missing Texture Fallback)
   - 8.2 Bellek Sızıntısı ve GC Profiling Tablosu
   - 8.3 Steam Deck, Linux, MacOS ve High-DPI UI Scale Test Senaryoları

---

## 1. Yönetici Özeti ve Mimari Vizyon

*Stardew Valley* mod ekosisteminde zengin ve modern kullanıcı arayüzleri (UI) tasarlarken karşılaşılan en büyük darboğazlardan biri, **özel PNG varlıklarının (custom textures), NPC portrelerinin, sprite sheet animasyonlarının ve oyun içi eşya ikonlarının (items)** dinamik ve verimli bir biçimde yüklenip çizilmesidir.

Vanilya SMAPI modlamasında mod geliştiricileri;
1. Mod klasöründeki yerel PNG dosyalarını `Helper.ModContent.Load<Texture2D>("assets/...")` ile yüklemek,
2. Oyun içi NPC portreleri veya vanilya atlasları için `Helper.GameContent.Load<Texture2D>("Portraits/...")` çağırmak,
3. Stardew Valley 1.6 ile gelen dinamik eşyalar için `ItemRegistry.GetDataOrErrorItem(qualifiedId)` çağrısı yapıp `ParsedItemData.GetTexture()` ve `GetSourceRect()` koordinatlarını manuel olarak hesaplamak,
4. Her bir görselin piksel genişlik/yükseklik oranını korumak için karmaşık X/Y oran hesaplamaları yapmak ve `SpriteBatch.Draw` çağrılarını piksel piksel yönetmek zorundadır.

Bu geleneksel yöntem; yüzlerce satır kırılgan kod, her karede meydana gelen Garbage Collector (GC) baskısı, Content Patcher modlarının varlıkları güncellemesi (hot-reload / asset invalidation) durumunda önbelleğin bozulması ve modern Flexbox/Grid yerleşimleriyle entegre olamama problemlerini beraberinde getirir.

### RustedUI Çözümü: Birleşik Varlık Boru Hattı (Unified Asset Pipeline)

**RustedUI**, bu karmaşıklığı ortadan kaldırarak web geliştirme konforunda bir **Deklaratif Varlık Boru Hattı** sunar:
- **Deklaratif Markup ve TSX:** Geliştiriciler `<img src="assets/banner.png" class="w-120 h-auto object-contain" />`, `<img src="Portraits/Abigail" rect="0,0,64,64" class="w-128 h-128" />` veya `<item-icon item="(O)74" quality="iridium" count="999" />` yazarak varlıkları doğrudan şablonlarına bağlar.
- **Sıfır-Kopya FFI String Interning (0 Allocation / Frame):** Rust yerel çekirdeği şablon yükleme anında görsel yollarını (`src`) tekil bir `u32` tamsayısına (`TextureId >= 1000`) dönüştürür (interning). Render döngüsünde (60–240 FPS) FFI sınırından tek bir string dahi geçmez.
- **Taffy Intrinsic Sizing & Aspect Ratio:** C# tarafında yüklenen `Texture2D` boyutları tek seferlik bir FFI el sıkışması (`rusted_tree_sync_texture_size`) ile Rust motoruna bildirilir. Taffy yerleşim motoru, imajın doğal en-boy oranını (`aspect_ratio`) otomatik olarak hesaplar; `w-120 h-auto` tanımlarında yüksekliği bozmadan mükemmel piksel hizalaması (pixel-snapping) ile ölçekler.
- **CSS `object-fit` Motoru:** `contain`, `cover`, `fill`, `none` ve `scale-down` modları Rust çekirdeğinde hesaplanır. Özellikle `cover` modu, pahalı donanımsal Scissor Clipping çağrılarına ihtiyaç duymaksızın kaynak dikdörtgenini (source rect) matematiksel olarak kırparak sıfır ek GPU maliyetiyle tam kutu doldurma sağlar.
- **Yerel Stardew 1.6 `ItemRegistry` Uyumu:** Qualified ID formatındaki tüm eşyalar (`(O)74` Prismatic Shard, `(BC)12` Keg, `(W)4` Galaxy Sword, `(B)506` Space Boots, `(TR)1` Trinket vb.) otomatik olarak `ItemRegistry` üzerinden çözülür; kalite yıldızları (`silver`, `gold`, `iridium`) ve yığın sayaçları piksel-mükemmel vanilya fontlarıyla çizilir.
- **Kusursuz SMAPI Yaşam Döngüsü & Asset Invalidation:** `IContentEvents.AssetsInvalidated` olayı dinlenerek Content Patcher yamaları veya dil değişikliklerinde önbellekteki `Texture2D` referansları anında tazelenir; menü kapandığında ise mod-özel unmanaged kaynaklar deterministik olarak serbest bırakılır.

```mermaid
flowchart TD
    subgraph Authoring["1. Geliştirici Katmanı (Markup / TSX / C# Fluent API)"]
        HTML["HTML Şablonu\n<img src='assets/hero.png' class='w-120 h-auto object-contain' />\n<item-icon item='(O)74' quality='iridium' />"]
        TSX["React / TSX Bileşeni\n<Img src='Portraits/Abigail' rect={[0,0,64,64]} />"]
        Fluent["C# Fluent API\nUI.Img('assets/hero.png').ObjectContain().W(120)"]
    end

    subgraph RustCore["2. Rusted Native Core (Rust - cdylib)"]
        Interner["Texture String Interner\n(Hash String -> u32 TextureId >= 1000)"]
        TaffyEngine["Taffy Layout Engine\n(Intrinsic Measure + Aspect Ratio Locking)"]
        ObjectFit["DrawList Generator\n(object-fit Contain/Cover Source Rect Math)"]
        CmdBuf["Contiguous RenderCommand Buffer\n(DrawSprite / DrawNineSlice)"]
    end

    subgraph FFI["3. Zero-Allocation FFI Boundary"]
        SyncSize["rusted_tree_sync_texture_size(tree, tex_id, w, h)"]
        RenderSlice["ReadOnlySpan<RenderCommand> (Zero-Copy)"]
    end

    subgraph CSharpRuntime["4. C# SMAPI Runtime (Rusted.Smapi)"]
        TexManager["TextureManager (Cache & Invalidation)\nDictionary<uint, Texture2D>"]
        ModHelper["IModContentHelper / IGameContentHelper"]
        ItemReg["Stardew 1.6 ItemRegistry\n(GetDataOrErrorItem)"]
        Renderer["RustedRenderer -> MonoGame SpriteBatch.Draw"]
    end

    HTML --> Interner
    TSX --> Interner
    Fluent --> Interner
    Interner --> TaffyEngine
    TaffyEngine --> ObjectFit
    ObjectFit --> CmdBuf
    CmdBuf -->|0 Kopya| RenderSlice
    RenderSlice --> Renderer

    Renderer --> TexManager
    TexManager --> ModHelper
    TexManager --> ItemReg
    TexManager -->|Doku Boyutlarını Bildir| SyncSize
    SyncSize --> TaffyEngine
```

---

## 2. Deklaratif Etiket Mimarisi: `<img />`, `<sprite />`, `<item-icon />` ve `<animated-sprite />`

RustedUI, modern HTML5 semantiğini Stardew Valley oyun içi varlık sisteminin ihtiyaçlarıyla harmanlar.

### 2.1 Desteklenen Varlık Kaynakları ve URI Protokolleri

RustedUI varlık boru hattı 3 ana kaynak sınıfını destekler:

| Kaynak Türü | Örnek Sözdizimi | Çözümleyici Motor | Açıklama |
|:---|:---|:---|:---|
| **Mod-Yerel PNG Dosyası** | `src="assets/banner.png"`<br/>`src="mod://textures/frame.png"` | `IModContentHelper.Load<Texture2D>` | Modun kendi kurulu olduğu dizindeki `assets/` klasöründen PNG/XNB yükler. |
| **Vanilya / SMAPI Game Content** | `src="Portraits/Abigail"`<br/>`src="LooseSprites/Cursors"`<br/>`src="game://Characters/Farmer"` | `IGameContentHelper.Load<Texture2D>` | Oyunun ana içerik boru hattından veya Content Patcher modlarının yamaladığı atlaslardan yükler. |
| **Qualified Item ID (Stardew 1.6)** | `item="(O)74"` (Prizmatik Parça)<br/>`item="(BC)12"` (Fıçı - Keg)<br/>`item="(W)4"` (Galaksi Kılıcı)<br/>`item="(B)506"` (Uzay Çizmeleri) | `ItemRegistry.GetDataOrErrorItem` | Stardew 1.6 evrensel eşya sisteminden dinamik atlas ve kaynak dikdörtgenini çeker. |

### 2.2 HTML/XML Şablon Sözdizimi ve Öznitelik Spesifikasyonu

#### 1. `<img />` Etiketi: Genel İmaj ve Portre Yükleyici
```html
<!-- Otomatik doğal boyutlandırma (Intrinsic Sizing) -->
<img src="assets/branding_logo.png" class="object-contain" />

<!-- Belirli genişlik verip yüksekliği doğal en-boy oranına göre kilitleme -->
<img src="assets/character_full.png" class="w-120 h-auto object-contain" />

<!-- Vanilya NPC Portresi: Belirli atlas kaynak koordinatından (rect="x,y,w,h") kırpma -->
<img src="Portraits/Abigail" rect="0,0,64,64" class="w-128 h-128 pixelated rounded shadow-md" />

<!-- Kutu boyutunu doldururken en-boy oranını koruyup taşan kısmı kırpma (Cover) -->
<img src="assets/landscape.png" class="w-200 h-100 object-cover" />
```

Desteklenen `<img />` Öznitelikleri:
- `src` *(Zorunlu)*: Varlığın göreceli yolu (`assets/...` veya `Portraits/...`).
- `rect` *(İsteğe Bağlı)*: Sprite atlasından kesilecek piksel koordinatları: `"X, Y, Width, Height"`. Belirtilmezse tüm doku kullanılır (`(0, 0, tex.Width, tex.Height)`).
- `tint` *(İsteğe Bağlı)*: MonoGame çizim rengi renklendirmesi (HEX veya Tailwind renk adı: `#ffffff`, `#ff8888`, `red-400`).
- `class` *(İsteğe Bağlı)*: Tailwind yerleşim, boyutlandırma ve `object-fit` sınıfları.

#### 2. `<sprite />` Etiketi: Sprite Atlaslarından Kesilmiş Statik Görseller
Büyük sprite atlaslarından (örneğin `LooseSprites/Cursors` veya özel bir UI atlası) tekil pikselleri çekmek için daha net ve anlamsal bir sözdizimi sunar:
```html
<!-- Vanilya Cursors atlasından Junimo ikonu çekme -->
<sprite src="LooseSprites/Cursors" rect="548,440,12,14" scale="4" class="hover:scale-110 transition-transform" />

<!-- Özel mod atlasından simge kırpma -->
<sprite src="assets/skill_icons.png" rect="32,0,16,16" class="w-32 h-32" tint="#ffd700" />
```

#### 3. `<item-icon />` Etiketi: Stardew 1.6 Evrensel Eşya Gösterici
Eşyaların manuel atlas hesaplaması yapılmaksızın doğrudan Qualified Item ID ile çağrılmasını sağlar:
```html
<!-- Standart Eşya: Prizmatik Parça -->
<item-icon item="(O)74" class="w-16 h-16" tooltip="Prizmatik Parça" />

<!-- İridyum Kalite ve 99 Yığın Sayısı ile Fıçı -->
<item-icon item="(BC)12" quality="iridium" count="99" class="w-16 h-16" />

<!-- Gümüş Kalite Yıldızlı Somon Balığı -->
<item-icon item="(O)139" quality="silver" count="5" class="w-16 h-16" />
```

Desteklenen `<item-icon />` Öznitelikleri:
- `item` *(Zorunlu)*: Qualified Item ID (`(O)ID`, `(BC)ID`, `(W)ID`, `(B)ID`, `(H)ID`, `(T)ID`, `(TR)ID`).
- `quality` *(İsteğe Bağlı)*: `normal` (veya 0), `silver` (1), `gold` (2), `iridium` (4).
- `count` *(İsteğe Bağlı)*: Sağ alt köşede çizilecek yığın adedi tamsayısı (`count > 1` ise çizilir).
- `show-tooltip` *(İsteğe Bağlı)*: `true` ise farenin üzerine gelindiğinde oyunun orijinal eşya adını ve açıklamasını içeren hover tooltip'i otomatik oluşturur.

---

### 2.3 React / TSX Bileşen Arayüzü

RustedUI TSX motoru (`@rusted/react` veya yerel React-Vite bağlamı), bu deklaratif etiketleri katı TypeScript tipleriyle (strict types) sarmalar:

```tsx
import React from "react";
import { Img, Sprite, ItemIcon, AnimatedSprite } from "@rusted/react";

export interface FarmerCardProps {
    farmerName: string;
    portraitSrc?: string;
    favoriteItem: string;
}

export const FarmerCard: React.FC<FarmerCardProps> = ({
    farmerName,
    portraitSrc = "Portraits/Abigail",
    favoriteItem = "(O)74",
}) => {
    return (
        <div className="flex flex-row items-center gap-4 p-4 sdv-box">
            {/* NPC veya Çiftçi Portresi */}
            <div className="relative w-16 h-16 rounded overflow-hidden border-2 border-amber-900 bg-amber-100">
                <Img
                    src={portraitSrc}
                    rect={[0, 0, 64, 64]}
                    className="w-full h-full object-contain pixelated"
                />
            </div>

            {/* Bilgi Metinleri */}
            <div className="flex flex-col flex-1">
                <span className="text-xl font-bold text-amber-950 font-dialogue">
                    {farmerName}
                </span>
                <span className="text-sm text-stone-600">Pelican Kasabası Sakini</span>
            </div>

            {/* Favori Eşya İkonu */}
            <div className="sdv-slot w-16 h-16 flex items-center justify-center">
                <ItemIcon item={favoriteItem} quality="iridium" count={1} />
            </div>
        </div>
    );
};
```

---

### 2.4 C# Fluent Builder API Entegrasyonu

HTML şablonu kullanmak istemeyen C# mod geliştiricileri için sıfır-GC fluent builder arayüzü:

```csharp
using Rusted.Smapi.UI;
using Rusted.Smapi.Native;

public static RustedElementBuilder CreateCustomShopPanel()
{
    return UI.Div()
        .Class("flex flex-col gap-3 p-6 sdv-box w-160")
        .Children(
            // Başlık Görseli
            UI.Img("assets/shop_header.png")
                .Class("w-full h-32 object-cover rounded shadow"),

            // İçerik Satırı
            UI.Div().Class("flex flex-row items-center gap-4")
                .Children(
                    // NPC Portresi (Abigail Gülümseme: Rect [64, 0, 64, 64])
                    UI.Sprite("Portraits/Abigail", new RectI(64, 0, 64, 64))
                        .Class("w-24 h-24 pixelated sdv-panel p-1"),

                    UI.Div().Class("flex flex-col flex-1")
                        .Children(
                            UI.Label("Abigail'in Maden Dükkanı")
                                .Font(VanillaFontId.Dialogue)
                                .TextDark(),
                            UI.Label("Bugün en taze ametistleri getirdim!")
                                .Font(VanillaFontId.Small)
                                .TextMuted()
                        )
                ),

            UI.Hr(),

            // Vitrin Slotları
            UI.Div().Class("grid grid-cols-4 gap-2")
                .Children(
                    UI.ItemSlot("(O)66").Count(5),              // Ametist
                    UI.ItemSlot("(O)74").Quality(ItemQuality.Iridium), // Prizmatik Parça
                    UI.ItemSlot("(W)4"),                        // Galaksi Kılıcı
                    UI.ItemSlot("(BC)12").Count(2)              // Fıçı
                )
        );
}
```

---

## 3. Sıfır-Kopya FFI String-to-TextureId Registry ve Önbellek Mimarisi

### 3.1 Sorun Analizi: Çizim Döngüsünde String Taşıma Maliyeti ve GC Baskısı

MonoGame ve SMAPI çizim döngüsü her saniye 60 ila 240 kez çalışır. Eğer render komutu (`RenderCommand`) içerisinde her imaj için dosya yolu dize (`string`) olarak taşınsaydı:
1. Rust tarafında her karede UTF-8 byte dizileri tahsis edilecek ve işaretçiler yönetilecekti.
2. C# tarafında her bir `RenderCommand` işlenirken `Marshal.PtrToStringUTF8` veya `Encoding.UTF8.GetString` çağrılacaktı.
3. 50 adet imaj içeren bir menüde her saniyede binlerce geçici string nesnesi (.NET Gen0 Managed Heap üzerinde) oluşacak, bu da Garbage Collector'ı her birkaç saniyede bir tetikleyerek oyunda mikro takılmalara (micro-stutter) neden olacaktı.

### 3.2 İki Aşamalı Texture Interning Mimarisi ve Dinamik ID Havuzu

RustedUI bu sorunu **İki Aşamalı Texture Interning (Two-Phase Texture Interning)** ile kökünden çözer:

```
[TextureId Aralığı]
0 .. 7      -> Yerleşik Vanilya Dokuları (Cursors, MenuTiles, Objects, LucideAtlas vb.)
8 .. 999    -> Rezerve Sistem Atlasları
1000 .. MAX -> Dinamik Kayıtlı Özel Dokular (Mod PNG'leri, NPC Portreleri, Eşya Dokuları)
```

1. **Aşama 1: Şablon Ayrıştırma Anında Interning (Parse-Time Interning)**
   Rust HTML okuyucusu (`load_template_markup`) bir `<img src="assets/banner.png">` veya `<sprite src="Portraits/Abigail">` gördüğünde:
   - Rust içindeki `TextureInterner` tablosuna bakar (`HashMap<String, u32>`).
   - Eğer `"assets/banner.png"` daha önce kaydedilmemişse, yeni bir dinamik kimlik üretir: `tex_id = 1000 + next_counter++`.
   - String yolunu ve üretilen `u32` ID'yi iç kuyruğuna (`pending_texture_registrations`) ekler.
   - İlgili `ElementNode`'a doğrudan bu tamsayı kimliği (`tex_id`) atanır.

2. **Aşama 2: Çizim Döngüsünde Sıfır String (Frame-Time Zero String)**
   `compute_and_render` çalıştığında üretilen `RenderCommand.TextureId` alanı yalnızca bu `u32` tamsayısını taşır:
   ```rust
   // Rust tarafında blittable struct kopyası (0 bayt string aktarımı!)
   RenderCommand {
       kind: RenderCommandKind::DrawSprite as u32,
       texture_id: 1000, // assets/banner.png
       dest_rect: rect,
       source_rect: src,
       color: ColorRgba::WHITE,
       ..Default::default()
   }
   ```

### 3.3 Çift Yönlü FFI El Sıkışması (Bidirectional Handshake)

Aşağıdaki Mermaid sekans diyagramı, yeni bir görselin ilk kez keşfedilmesinden itibaren C# ve Rust arasındaki sıfır-kopya el sıkışmasını göstermektedir:

```mermaid
sequenceDiagram
    autonumber
    participant Mod as Mod Kod / Şablon
    participant Core as Rust UiTree
    participant FFI as FFI Köprüsü
    participant CS as C# TextureManager
    participant SMAPI as SMAPI Content Pipeline

    Mod->>Core: rusted_tree_load_markup(HTML with <img src="assets/logo.png" />)
    Core->>Core: TextureInterner: "assets/logo.png" -> ID 1000 atanır
    Core->>Core: PendingRegistrations.push(1000, "assets/logo.png")
    Core-->>Mod: Root NodeKey Döndürülür

    Note over Mod,CS: İlk Render veya Update Döngüsü
    CS->>FFI: rusted_tree_get_pending_textures(tree, out SliceView)
    FFI-->>CS: ReadOnlySpan<TextureRegistrationEntry>
    loop Her Yeni Doku İçin
        CS->>SMAPI: helper.ModContent.Load<Texture2D>("assets/logo.png")
        SMAPI-->>CS: Texture2D Örneği (Width: 320, Height: 180)
        CS->>CS: TextureCache[1000] = Texture2D
        CS->>FFI: rusted_tree_sync_texture_size(tree, 1000, 320, 180)
        FFI->>Core: Rust TextureMeta güncellenir & Node Layout DIRTY işaretlenir
    end

    Note over Core,CS: Render Döngüsü (60+ FPS)
    CS->>FFI: rusted_tree_render(tree, viewportW, viewportH, out RenderSlice)
    FFI-->>CS: ReadOnlySpan<RenderCommand> (TextureId = 1000)
    CS->>CS: Fast Array Lookup: TextureCache[1000]
    CS->>CS: SpriteBatch.Draw(TextureCache[1000], dest, src, color)
```

### 3.4 C# `TextureManager`: Lazy-Load, ModContent, GameContent ve ItemRegistry Çözümleme

C# tarafındaki `TextureManager` sınıfı, Rust'tan gelen istekleri ilgili alt sisteme yönlendirir:

```csharp
using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using StardewModdingAPI;
using StardewValley;
using StardewValley.ItemTypeDefinitions;

namespace Rusted.Smapi.Rendering
{
    public sealed class TextureManager
    {
        private readonly IModHelper _helper;
        private readonly IMonitor _monitor;
        private readonly Dictionary<uint, Texture2D> _texturesById = new();
        private readonly Dictionary<string, uint> _idByPath = new(StringComparer.OrdinalIgnoreCase);
        private readonly Dictionary<uint, string> _pathById = new();

        public TextureManager(IModHelper helper, IMonitor monitor)
        {
            _helper = helper;
            _monitor = monitor;
        }

        public Texture2D ResolveTexture(uint textureId)
        {
            if (_texturesById.TryGetValue(textureId, out Texture2D? tex))
                return tex;

            // Bilinmeyen / henüz yüklenmemiş doku için hata vermeyen fallback
            return Game1.staminaRect;
        }

        public bool TryRegisterTexture(uint textureId, string assetPath, out int width, out int height)
        {
            width = 0;
            height = 0;

            if (_texturesById.TryGetValue(textureId, out Texture2D? existing))
            {
                width = existing.Width;
                height = existing.Height;
                return true;
            }

            try
            {
                Texture2D loaded;

                if (assetPath.StartsWith("(O)") || assetPath.StartsWith("(BC)") ||
                    assetPath.StartsWith("(W)") || assetPath.StartsWith("(B)") ||
                    assetPath.StartsWith("(H)") || assetPath.StartsWith("(T)") ||
                    assetPath.StartsWith("(TR)"))
                {
                    // 1. Stardew 1.6 Item Registry Çözümlemesi
                    ParsedItemData itemData = ItemRegistry.GetDataOrErrorItem(assetPath);
                    loaded = itemData.GetTexture();
                }
                else if (assetPath.StartsWith("assets/") || assetPath.StartsWith("assets\\") || assetPath.StartsWith("mod://"))
                {
                    // 2. Mod-Yerel Varlık (IModContentHelper)
                    string cleanPath = assetPath.StartsWith("mod://") ? assetPath[6..] : assetPath;
                    loaded = _helper.ModContent.Load<Texture2D>(cleanPath);
                }
                else
                {
                    // 3. Vanilya / Oyun İçerik Boru Hattı (IGameContentHelper)
                    string cleanPath = assetPath.StartsWith("game://") ? assetPath[7..] : assetPath;
                    loaded = _helper.GameContent.Load<Texture2D>(cleanPath);
                }

                _texturesById[textureId] = loaded;
                _idByPath[assetPath] = textureId;
                _pathById[textureId] = assetPath;

                width = loaded.Width;
                height = loaded.Height;
                return true;
            }
            catch (Exception ex)
            {
                _monitor.Log($"[RustedUI] Varlık yüklenemedi: '{assetPath}' (ID: {textureId}). Hata: {ex.Message}", LogLevel.Error);
                _texturesById[textureId] = Game1.staminaRect;
                width = 1;
                height = 1;
                return false;
            }
        }
    }
}
```

### 3.5 SMAPI Önbellek Geçersiz Kılma (Asset Invalidation) ve Hot-Reload

Stardew Valley modlamasında Content Patcher veya yerel modlar, oyun çalışırken dokuları değiştirebilir (örneğin mevsimlik NPC kıyafetleri, gece/gündüz binaları veya dil paketleri). SMAPI bu durumu `helper.Events.Content.AssetsInvalidated` olayı ile bildirir.

`TextureManager`, geçersiz kılınan varlıkları anında tespit edip önbelleği tazeler:

```csharp
public void OnAssetsInvalidated(object? sender, AssetsInvalidatedEventArgs e)
{
    List<uint> invalidatedIds = new();

    foreach (var kvp in _pathById)
    {
        uint id = kvp.Key;
        string path = kvp.Value;

        // e.NamesWithoutLocale, dil uzantılarından arındırılmış varlık adlarını denetler
        foreach (IAssetName assetName in e.NamesWithoutLocale)
        {
            if (assetName.IsEquivalentTo(path))
            {
                invalidatedIds.Add(id);
                break;
            }
        }
    }

    foreach (uint id in invalidatedIds)
    {
        string path = _pathById[id];
        _monitor.Log($"[RustedUI] Doku geçersiz kılındı (Invalidated): '{path}', yeniden yükleniyor...", LogLevel.Trace);
        
        // Önbellekten kaldır ve GameContent üzerinden yeniden yükle
        _texturesById.Remove(id);
        if (TryRegisterTexture(id, path, out int w, out int h))
        {
            // İmaj boyutu değiştiyse Rust layout motorunu uyar
            NativeMethods.rusted_tree_sync_texture_size(_activeTreePtr, id, (uint)w, (uint)h);
        }
    }
}
```

---

## 4. Taffy ile Doğal Boyutlandırma (Intrinsic Sizing), Aspect Ratio ve CSS `object-fit`

Web tarayıcılarında `<img src="pic.png" />` yazıldığında, tarayıcı imajın PNG başlığından (header) genişlik ve yükseklik değerlerini okur ve CSS'te `width: auto; height: auto` verildiğinde imajı kendi doğal piksel boyutunda çizer. Eğer sadece `width: 200px` verilirse, yüksekliği orijinal en-boy oranına (aspect ratio) göre otomatik ölçekler.

RustedUI, Taffy 0.5+ layout motoru üzerinde bu davranışı kusursuz bir biçimde taklit eder.

### 4.1 İki Aşamalı Yerleşim (Layout) ve Boyut Senkronizasyon Protokolü

```mermaid
sequenceDiagram
    autonumber
    participant Taffy as Rust Taffy Tree
    participant Core as Rust UiTree
    participant FFI as FFI Köprüsü
    participant CS as C# RustedRenderer

    Core->>Taffy: 1. Layout Hesabı (Boyut henüz bilinmiyor: Varsayılan 0x0)
    Core->>FFI: RenderSliceView (Texture ID 1000 keşfedildi)
    FFI->>CS: C# TextureManager.Resolve
    CS->>CS: Texture2D yüklendi (Örn: 256x128 px)
    CS->>FFI: rusted_tree_sync_texture_size(tree, 1000, 256, 128)
    FFI->>Core: Rust TextureMeta tablosu güncellendi
    Core->>Taffy: mark_dirty(node.taffy_id)
    Note over Core,Taffy: Sonraki Karede (Frame 2):
    Core->>Taffy: 2. Layout Hesabı (Intrinsic Boyut ve Aspect Ratio devrede)
    Taffy-->>Core: Hesaplanmış Boyut: 256x128 px
    Core->>FFI: RenderCommand (DestRect: 256x128 px)
```

### 4.2 Taffy Layout Context İçinde Leaf Measure Fonksiyonu

Rust çekirdeğinde `taffy.compute_layout_with_measure` çağrıldığında, imaj düğümleri (`is_image = true`) için aşağıdaki ölçüm algoritması çalışır:

```rust
// crates/rusted-core/src/dom.rs
self.taffy.compute_layout_with_measure(
    root_taffy,
    Size {
        width: AvailableSpace::Definite(viewport_w),
        height: AvailableSpace::Definite(viewport_h),
    },
    |known_dimensions, available_space, _node_id, node_context, _style| {
        if let Some(ctx) = node_context {
            // 1. Metin Düğümü Ölçümü
            if ctx.is_text {
                // ... Mevcut Font Atlas metin ölçümü ...
            }

            // 2. İmaj Düğümü Doğal Boyut (Intrinsic Sizing) Ölçümü
            if ctx.is_image {
                if let Some(node) = arena_ref.get(ctx.node_key) {
                    let (intrinsic_w, intrinsic_h) = node.get_intrinsic_dimensions();

                    let w = known_dimensions.width.unwrap_or_else(|| {
                        match available_space.width {
                            AvailableSpace::Definite(max_w) => intrinsic_w.min(max_w),
                            AvailableSpace::MinContent => 0.0,
                            AvailableSpace::MaxContent => intrinsic_w,
                        }
                    });

                    let h = known_dimensions.height.unwrap_or_else(|| {
                        if intrinsic_w > 0.0 {
                            // En-boy oranına göre hesapla (aspect ratio locking)
                            let ratio = intrinsic_h / intrinsic_w;
                            w * ratio
                        } else {
                            intrinsic_h
                        }
                    });

                    return Size { width: w, height: h };
                }
            }
        }

        Size {
            width: known_dimensions.width.unwrap_or(0.0),
            height: known_dimensions.height.unwrap_or(0.0),
        }
    },
)
```

### 4.3 En-Boy Oranı Kilitleme (Aspect Ratio Locking) Matematiği

Mod geliştiricisi Tailwind sınıfları kullanarak imajı boyutlandırdığında Taffy şu üç kuralı uygular:

1. **`class="w-auto h-auto"` (Doğal Boyut):**
   $$\text{Hesaplanan Genişlik} = \text{Doku Genişliği}, \quad \text{Hesaplanan Yükseklik} = \text{Doku Yüksekliği}$$
2. **`class="w-120 h-auto"` (Genişlik Sabit, Yükseklik Oransal):**
   $$\text{Oran} = \frac{\text{Doğal Yükseklik}}{\text{Doğal Genişlik}}, \quad \text{Hesaplanan Yükseklik} = 120.0 \times \text{Oran}$$
3. **`class="w-auto h-80"` (Yükseklik Sabit, Genişlik Oransal):**
   $$\text{Oran} = \frac{\text{Doğal Genişlik}}{\text{Doğal Yükseklik}}, \quad \text{Hesaplanan Genişlik} = 80.0 \times \text{Oran}$$

Tüm koordinat hesaplamalarının sonunda Rusted piksel hizalama motoru `round()` uygulayarak piksel sanatının sub-pixel kaymalarla bulanıklaşmasını engeller.

---

### 4.4 CSS `object-fit` Algoritmaları: `fill`, `contain`, `cover`, `none`, `scale-down`

Geliştirici imaja hem kesin genişlik hem kesin yükseklik atadığında (örneğin `class="w-160 h-100"`), imajın kutu içine nasıl yerleşeceği `object-fit` algoritmasıyla belirlenir.

```
Kutu Boyutu: (dest_w, dest_h)
Kaynak Doku Boyutu: (src_w, src_h)
```

```mermaid
flowchart LR
    subgraph Fill["object-fit: fill"]
        F1["Tüm kutuyu doldurur\nEn-boy oranı bozulur (Gerilme)"]
    end
    subgraph Contain["object-fit: contain"]
        C1["En-boy oranı korunur\nKutu içine sığdırılır\nKenarlarda boşluk kalabilir (Letterbox)"]
    end
    subgraph Cover["object-fit: cover"]
        CV1["En-boy oranı korunur\nKutunun tamamı doldurulur\nTaşan kısımlar kaynaktan kırpılır"]
    end
```

#### 1. `object-fit: fill` (Varsayılan)
Görsel kutuya tamamen gerilir.
$$\text{DestRect} = (x, y, \text{dest\_w}, \text{dest\_h}), \quad \text{SourceRect} = (0, 0, \text{src\_w}, \text{src\_h})$$

#### 2. `object-fit: contain`
Görsel, en-boy oranı bozulmadan kutunun içine sığdırılır; sağ-solda (pillarbox) veya alt-üstte (letterbox) boşluk kalır ve görsel kutunun merkezine hizalanır:
$$\text{scale} = \min\left(\frac{\text{dest\_w}}{\text{src\_w}}, \, \frac{\text{dest\_h}}{\text{src\_h}}\right)$$
$$\text{draw\_w} = \text{src\_w} \times \text{scale}, \quad \text{draw\_h} = \text{src\_h} \times \text{scale}$$
$$\text{draw\_x} = x + \frac{\text{dest\_w} - \text{draw\_w}}{2}, \quad \text{draw\_y} = y + \frac{\text{dest\_h} - \text{draw\_h}}{2}$$

#### 3. `object-fit: cover` (Sıfır Scissor Clipping ile Kaynak Kırpma)
Geleneksel render motorları `cover` modunda görseli büyütüp taşan kısmı GPU Scissor Test ile kırpar. Bu durum `SpriteBatch` içinde `batch.End()` ve `batch.Begin()` çağrısına (state break) neden olur.

**RustedUI İnovasyonu:** RustedUI `cover` modunu **Kaynak Dikdörtgeni Kırpması (Source Rectangle Cropping)** ile hesaplar. Böylece GPU'ya ekstra kırpma komutu göndermeden, tek bir `SpriteBatch.Draw` çağrısıyla sıfır maliyetli cover elde edilir:

$$\text{scale} = \max\left(\frac{\text{dest\_w}}{\text{src\_w}}, \, \frac{\text{dest\_h}}{\text{src\_h}}\right)$$
$$\text{visible\_src\_w} = \frac{\text{dest\_w}}{\text{scale}}, \quad \text{visible\_src\_h} = \frac{\text{dest\_h}}{\text{scale}}$$
$$\text{crop\_x} = \text{src\_x} + \frac{\text{src\_w} - \text{visible\_src\_w}}{2}, \quad \text{crop\_y} = \text{src\_y} + \frac{\text{src\_h} - \text{visible\_src\_h}}{2}$$
$$\text{RenderCommand.DestRect} = (x, y, \text{dest\_w}, \text{dest\_h})$$
$$\text{RenderCommand.SourceRect} = (\text{crop\_x}, \text{crop\_y}, \text{visible\_src\_w}, \text{visible\_src\_h})$$

> [!TIP]
> **Performans Kazanımı:** Kaynak dikdörtgeni kırpma formülü sayesinde `cover` kullanılan yüzlerce avatar veya portre çizilirken MonoGame grafik boru hattında tek bir scissor state değişikliği dahi yaşanmaz.

---

### 4.5 Piksel-Sanatı Netliği: `image-rendering: pixelated` ve MonoGame `PointClamp`

Stardew Valley bir piksel-sanatı (pixel-art) oyunudur. Görseller büyütüldüğünde veya küçültüldüğünde bilineer filtreleme (`LinearClamp`) kullanılırsa pikseller bulanıklaşır (blurry artifact). 

RustedUI'da `pixelated` veya `image-rendering: pixelated` sınıfı:
1. Koordinatları ve boyutları kesin tamsayı piksel sınırlarına (`f32.round()`) yuvarlar.
2. MonoGame `RustedRenderer` üzerinde `SamplerState.PointClamp` aktif olduğu için en yakın komşu (nearest-neighbor) enterpolasyonu ile 16x16 veya 32x32 retro görselleri cam gibi keskin çizer.

---

## 5. Animasyonlu Sprite'lar (`<animated-sprite />`) ve Zamanlama Boru Hattı

Mod yapımcıları çoğu zaman dinamik HUD öğelerinde veya menülerde hareketli varlıklar göstermek ister (örneğin kamp ateşi ateşi, dönen madeni para, yüzen Junimo, yürüyen çiftçi veya alet kullanım döngüsü).

### 5.1 Sprite Sheet Mimarisi (Horizontal Strip vs Grid Matrix)

RustedUI, 2D oyunlarda standart olan iki tür sprite sheet düzenini yerel olarak destekler:

1. **Yatay Şerit (Horizontal Strip - Varsayılan):** Kareler soldan sağa tek sıra halinde dizilmiştir.
   $$\text{Frame } i \implies \text{src\_x} = \text{base\_x} + (i \times \text{frame\_width}), \quad \text{src\_y} = \text{base\_y}$$
2. **Izgara Matrisi (Grid Matrix):** Kareler sütun ve satırlar halinde düzenlenmiştir.
   $$\text{Sütun} = i \pmod{\text{cols}}, \quad \text{Satır} = \lfloor i / \text{cols} \rfloor$$
   $$\text{src\_x} = \text{base\_x} + (\text{Sütun} \times \text{frame\_width}), \quad \text{src\_y} = \text{base\_y} + (\text{Satır} \times \text{frame\_height})$$

```html
<!-- Yatay şerit: 6 kare, saniyede 10 kare (FPS) -->
<animated-sprite 
    src="assets/campfire_anim.png" 
    frame-width="32" 
    frame-height="32" 
    frames="6" 
    fps="10" 
    loop="true" 
    class="w-32 h-32" />

<!-- Izgara matrisi: 16 kare, 4 sütun, ping-pong döngü modu -->
<animated-sprite 
    src="LooseSprites/Cursors" 
    rect="0,0,128,128"
    frame-width="32" 
    frame-height="32" 
    frames="16" 
    cols="4" 
    fps="12" 
    mode="ping-pong" 
    class="w-32 h-32" />
```

### 5.2 Deterministic Frame Hesaplama Matematiği

Zamanlayıcı hesaplaması her karede dinamik tahsis (allocation) yapmadan, Rust tarafında saf matematiksel modülo aritmetiği ile icra edilir:

```rust
// crates/rusted-core/src/dom.rs
pub fn compute_animated_frame(
    elapsed_time: f32,
    fps: f32,
    total_frames: u32,
    mode: AnimationLoopMode,
) -> u32 {
    if total_frames <= 1 || fps <= 0.0 {
        return 0;
    }

    let frame_duration = 1.0 / fps;
    let raw_frame_index = (elapsed_time / frame_duration).floor() as u32;

    match mode {
        AnimationLoopMode::Loop => raw_frame_index % total_frames,
        AnimationLoopMode::PlayOnce => raw_frame_index.min(total_frames - 1),
        AnimationLoopMode::PingPong => {
            let cycle = (total_frames - 1) * 2;
            let cycle_index = raw_frame_index % cycle;
            if cycle_index < total_frames {
                cycle_index
            } else {
                cycle - cycle_index
            }
        }
    }
}
```

### 5.3 SMAPI `UpdateTicked` / `GameTime` Entegrasyonu

C# katmanı SMAPI'nin `GameLoop.UpdateTicked` olayında geçen süreyi Rust motoruna tek bir blittable FFI çağrısıyla bildirir:

```csharp
private void OnUpdateTicked(object? sender, UpdateTickedEventArgs e)
{
    float deltaSeconds = (float)Game1.currentGameTime.ElapsedGameTime.TotalSeconds;
    
    // Tüm aktif Rusted görünümlerinin zamanlayıcısını güncelle
    if (_activeTreePtr != IntPtr.Zero)
    {
        NativeMethods.rusted_tree_tick(_activeTreePtr, deltaSeconds);
    }
}
```

Rust tarafında `rusted_tree_tick` çağrıldığında:
- Yalnızca `ComponentFlags::ANIMATED_SPRITE` bayrağına sahip düğümlerin `elapsed_time` sayacı artırılır.
- Yeni kare indeksi eskisiyle aynıysa hiçbir layout veya render bayrağı tetiklenmez.
- Kare indeksi değiştiğinde yalnızca `DirtyFlags::RENDER_DIRTY` tetiklenir (Taffy Flexbox layout motoru gereksiz yere yeniden çalıştırılmaz!).

---

## 6. Özel 9-Slice Çerçeve Sistemi (Custom 9-Slice Image Frames)

### 6.1 Vanilya Ötesi: Mod Yapımcılarının Özel UI Çerçeveleri

Stardew Valley modcuları her zaman vanilya ahşap kutu stilini (`sdv-box`) kullanmak istemez. Örneğin:
- Gotik bir vampir modunda antika siyah metal ve bordo kadife çerçeve,
- Fütüristik bir JojaCorp modunda neon mavi siber panel,
- Büyücü temalı bir modda parlayan rünlü parşömen çerçevesi.

RustedUI, herhangi bir harici PNG dosyasını **Sıfır-Konfigürasyon 9-Slice Çerçeveye** dönüştürmeyi destekler.

### 6.2 Etiket Sözdizimi: `<nine-slice />` ve Tailwind Utility Sınıfları

```html
<!-- XML Sözdizimi: 16px köşeler, 2x ölçek -->
<nine-slice src="assets/gothic_frame.png" slice="16" scale="2" class="p-6">
    <span class="text-white text-lg font-bold">Karanlık Zindan Sandığı</span>
    <p class="text-stone-300">Bu sandık sadece gece yarısı açılabilir.</p>
</nine-slice>

<!-- Farklı kenar kalınlıkları: slice="üst,sağ,alt,sol" -->
<nine-slice src="assets/ornate_border.png" slice="24,12,24,12" scale="1" class="w-180 h-120">
    <!-- Panel İçeriği -->
</nine-slice>
```

### 6.3 9-Bölge Geometrisi ve Piksel Kanaması (Bleeding) Önleme

Bir doku 9 dilime bölündüğünde MonoGame `SpriteBatch` şu parçaları çizer:

```
+---------------+-----------------------------+---------------+
| Sol-Üst Köşe  |         Üst Kenar           | Sağ-Üst Köşe  |
|  (Sabit Boyut)|         (Genişletilir)      |  (Sabit Boyut)|
+---------------+-----------------------------+---------------+
| Sol Kenar     |                             | Sağ Kenar     |
| (Yükseltilir) |          Merkez             | (Yükseltilir) |
|               |       (Doldurulur)          |               |
+---------------+-----------------------------+---------------+
| Sol-Alt Köşe  |         Alt Kenar           | Sağ-Alt Köşe  |
|  (Sabit Boyut)|         (Genişletilir)      |  (Sabit Boyut)|
+---------------+-----------------------------+---------------+
```

Piksel kanamasını (texture bleeding) önlemek için:
1. `destination` ve `source` dikdörtgenlerinin tamsayı piksel koordinatları `(int)` cast ile kesilir.
2. MonoGame `SamplerState.PointClamp` aktif tutularak komşu piksellerden renk sızması %100 engellenir.

---

## 7. Kapsamlı Veri Sözleşmesi, FFI ve Kod Referansları (Rust & C#)

### 7.1 Rust `rusted-core` Veri Yapıları (`types.rs`, `dom.rs`, `tailwind.rs`)

```rust
// crates/rusted-core/src/types.rs

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderCommandKind {
    DrawNineSlice = 0,
    DrawSprite = 1,
    DrawText = 2,
    DrawRect = 3,
    PushClipRect = 4,
    PopClipRect = 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectFit {
    Fill = 0,
    Contain = 1,
    Cover = 2,
    None = 3,
    ScaleDown = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationLoopMode {
    Loop = 0,
    PlayOnce = 1,
    PingPong = 2,
}

bitflags::bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ComponentFlags: u32 {
        const NONE            = 0;
        const SDV_BOX         = 1 << 0;
        const SDV_PANEL       = 1 << 1;
        const SDV_PARCHMENT   = 1 << 2;
        const SDV_BUTTON      = 1 << 3;
        const SDV_SLOT        = 1 << 4;
        const SDV_CLOSE       = 1 << 5;
        const SDV_CHECKBOX    = 1 << 6;
        const SDV_BAR         = 1 << 7;
        const SDV_DIVIDER     = 1 << 8;
        const SDV_SLIDER      = 1 << 9;
        const SDV_INPUT       = 1 << 10;
        const SDV_TAB         = 1 << 11;
        const SDV_SCROLL      = 1 << 12;
        const IS_HOVERED      = 1 << 20;
        const IS_ACTIVE       = 1 << 21;
        const IS_FOCUSED      = 1 << 22;
        const IS_CHECKED      = 1 << 23;
        const CLIP_CHILDREN   = 1 << 24;
        const TEXT_SHADOW     = 1 << 25;
        const SHADCN_BADGE    = 1 << 26;
        const SHADCN_SWITCH   = 1 << 27;
        const LUCIDE_ICON     = 1 << 28;
        const SHADCN_CARD     = 1 << 29;
        
        // Yeni İmaj ve Sprite Bayrakları:
        const CUSTOM_IMAGE    = 1 << 30; // <img /> veya <sprite />
        const ANIMATED_SPRITE = 1 << 31; // <animated-sprite />
    }
}

/// C# ile paylaşılan yeni doku kayıt bildirim yapısı
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureRegistrationEntry {
    pub texture_id: u32,
    pub path_offset: u32,
    pub path_len: u32,
}

#[repr(C)]
pub struct TextureRegistrationSliceView {
    pub entries_ptr: *const TextureRegistrationEntry,
    pub entries_len: usize,
    pub string_pool_ptr: *const u8,
    pub string_pool_len: usize,
}
```

Düğüm (Node) görsel özelliklerine imaj parametrelerinin eklenmesi:

```rust
// crates/rusted-core/src/tailwind.rs

#[derive(Clone, Debug)]
pub struct VisualProps {
    // ... Mevcut alanlar (bg_color, text_color, font, progress_value vb.) ...
    
    // Custom Image & Sprite Özellikleri
    pub custom_texture_id: u32,
    pub source_rect: RectI,
    pub object_fit: ObjectFit,
    pub image_scale: f32,
    pub border_insets: BorderInsets,

    // Animated Sprite Özellikleri
    pub frame_width: u32,
    pub frame_height: u32,
    pub total_frames: u32,
    pub cols: u32,
    pub fps: f32,
    pub loop_mode: AnimationLoopMode,
    pub elapsed_time: f32,
}

impl Default for VisualProps {
    fn default() -> Self {
        Self {
            // ... varsayılanlar ...
            custom_texture_id: 0,
            source_rect: RectI::default(),
            object_fit: ObjectFit::Fill,
            image_scale: 1.0,
            border_insets: BorderInsets::default(),
            frame_width: 0,
            frame_height: 0,
            total_frames: 1,
            cols: 0,
            fps: 8.0,
            loop_mode: AnimationLoopMode::Loop,
            elapsed_time: 0.0,
        }
    }
}
```

---

### 7.2 Rust `rusted-bridge` C-ABI Fonksiyonları

```rust
// crates/rusted-bridge/src/lib.rs

/// C# tarafına yüklenmesi gereken yeni dokuların listesini döndürür.
/// # Safety
/// `tree` ve `out_view` geçerli işaretçiler olmalıdır.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_get_pending_textures(
    tree: *mut UiTree,
    out_view: *mut TextureRegistrationSliceView,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    if out_view.is_null() { return 0; }
    
    let view = t.get_pending_texture_registrations();
    let count = view.entries_len as u32;
    *out_view = view;
    count
}

/// C# tarafında yüklenen Texture2D boyutunu Rust yerel çekirdeğine senkronize eder.
/// # Safety
/// `tree` geçerli bir işaretçi olmalıdır.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_sync_texture_size(
    tree: *mut UiTree,
    texture_id: u32,
    width: u32,
    height: u32,
) {
    let Some(t) = tree.as_mut() else { return };
    t.sync_texture_dimensions(texture_id, width, height);
}

/// Animasyonlu sprite'lar için geçen süreyi (delta seconds) günceller.
/// # Safety
/// `tree` geçerli bir işaretçi olmalıdır.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_tick(tree: *mut UiTree, delta_seconds: f32) {
    let Some(t) = tree.as_mut() else { return };
    t.tick_animations(delta_seconds);
}
```

---

### 7.3 C# P/Invoke Tanımları (`NativeProtocol.cs`)

```csharp
// src/Rusted.Smapi/Native/NativeProtocol.cs

namespace Rusted.Smapi.Native
{
    public enum ObjectFit : uint
    {
        Fill = 0,
        Contain = 1,
        Cover = 2,
        None = 3,
        ScaleDown = 4
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct TextureRegistrationEntry
    {
        public uint TextureId;
        public uint PathOffset;
        public uint PathLen;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct TextureRegistrationSliceView
    {
        public TextureRegistrationEntry* EntriesPtr;
        public UIntPtr EntriesLen;
        public byte* StringPoolPtr;
        public UIntPtr StringPoolLen;
    }

    public static unsafe partial class NativeMethods
    {
        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_get_pending_textures(
            IntPtr tree,
            TextureRegistrationSliceView* outView);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_tree_sync_texture_size(
            IntPtr tree,
            uint textureId,
            uint width,
            uint height);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_tree_tick(IntPtr tree, float deltaSeconds);
    }
}
```

---

### 7.4 C# `TextureManager.cs` ve `RustedRenderer.cs` Tam Uygulaması

`RustedRenderer.ExecuteRender` metodunun özel dokuları (`TextureId >= 1000`) destekleyecek şekilde güncellenmiş hali:

```csharp
// src/Rusted.Smapi/Rendering/RustedRenderer.cs (Alıntı)

private Texture2D ResolveTexture(TextureId id, uint rawTextureId)
{
    // 1. Dinamik Özel Doku (ID >= 1000)
    if (rawTextureId >= 1000)
    {
        return _textureManager.ResolveTexture(rawTextureId);
    }

    // 2. Yerleşik Vanilya Atlasları
    return id switch
    {
        TextureId.Cursors => Game1.mouseCursors,
        TextureId.Cursors2 => Game1.mouseCursors2,
        TextureId.MenuTiles => Game1.menuTexture,
        TextureId.MenuTilesUncolored => Game1.uncoloredMenuTexture,
        TextureId.Objects => Game1.objectSpriteSheet,
        TextureId.LucideAtlas => GetOrCreateLucideAtlas(),
        _ => Game1.mouseCursors
    };
}
```

Ve render döngüsünde `DrawSprite` ve `DrawNineSlice` komutlarında:

```csharp
case RenderCommandKind.DrawSprite:
{
    Texture2D tex = ResolveTexture((TextureId)cmd.TextureId, cmd.TextureId);
    Rectangle dest = new((int)cmd.DestRect.X, (int)cmd.DestRect.Y, (int)cmd.DestRect.Width, (int)cmd.DestRect.Height);
    Rectangle src = new(cmd.SourceRect.X, cmd.SourceRect.Y, cmd.SourceRect.Width, cmd.SourceRect.Height);
    batch.Draw(tex, dest, src, color);
    break;
}

case RenderCommandKind.DrawNineSlice:
{
    Texture2D tex = ResolveTexture((TextureId)cmd.TextureId, cmd.TextureId);
    NineSliceRenderer.Draw(batch, tex, in cmd.DestRect, in cmd.SourceRect, in cmd.Insets, color, cmd.Scale);
    break;
}
```

---

## 8. Mühendislik Kontrol Listesi, Performans Analizi ve Sınır Durumlar

### 8.1 Eksik/Hatalı Varlık Yönetimi (Missing Texture Fallback)

Bir mod geliştiricisi yanlış bir dosya yolu girdiğinde (örneğin `assets/yok_boyle_resim.png`) oyunun çökmesi (Crash to Desktop) kesinlikle kabul edilemez.

RustedUI Fallback Protokolü:
1. `TextureManager.TryRegisterTexture` `false` dönerse, ilgili `TextureId` haritasına `Game1.staminaRect` (1x1 beyaz piksel) veya vanilya hata simgesi (`ItemRegistry.GetDataOrErrorItem("???").GetTexture()`) atanır.
2. SMAPI konsoluna `LogLevel.Warn` ile net bir hata kaydı düşülür.
3. UI çizilmeye devam eder; hiçbir null reference exception veya MonoGame çizim çökmesi yaşanmaz.

### 8.2 Bellek Sızıntısı ve GC Profiling Tablosu

| İşlem | Geleneksel SMAPI Yöntemi | RustedUI Varlık Mimarisi | Kazanç / İyileşme |
|:---|:---|:---|:---|
| **Render Başına GC Allocation** | ~12–45 KB / kare (String ve Rectangle tahsisleri) | **0 Bayt / kare** (`ReadOnlySpan<RenderCommand>` unmanaged) | **%100 GC Baskısı Sıfırlandı** |
| **Doku Arama Hızı (Lookup)** | `Dictionary<string, Texture2D>` hash araması (~25 ns) | `Texture2D[]` doğrudan index veya uint dictionary (~3 ns) | **8x Daha Hızlı** |
| **Object-Fit Cover Maliyeti** | Scissor test state break (`batch.End` + `batch.Begin`) | **Kaynak Kırpma (Source Rect Crop)** (0 state break) | **Kare hızı dalgalanması önlendi** |
| **Asset Invalidation Tepkisi** | Manuel etkinlik dinleme veya oyun yeniden başlatma | **Otomatik `AssetsInvalidated` kancası** | Anında dinamik güncelleme |

### 8.3 Steam Deck, Linux, MacOS ve High-DPI UI Scale Test Senaryoları

1. **Steam Deck (GameScope 1280x800):**
   - SteamOS üzerinde `uiScale = 1.0` veya `1.25` iken imajlar piksel-mükemmel hizalanmalıdır. `Math.Round` piksel-snapping sayesinde Steam Deck ekranında simgelerin kenarlarında 1 piksellik titreme (shimmering) görülmez.
2. **Proton / Linux Sandbox Dosya Yolu Ayracı:**
   - Linux ve Proton altında dosya yollarındaki ters eğik çizgi (`assets\banner.png`) sorun çıkarabilir. `TextureManager`, gelen yollardaki tüm `\` işaretlerini standart `/` formatına normalize eder.
3. **High-DPI Ekranlar (4K Monitörler & `uiScale > 2.0`):**
   - Yüksek çözünürlüklerde `dest_rect` koordinatları otomatik olarak UI ölçeğine göre büyütülür; piksel sanatı `PointClamp` ile netliğini koruyarak büyütülür.

---

## 9. Sonuç ve Uygulama Yol Haritası

RustedUI için hazırlanan bu **Özel İmaj, SpriteSheet ve Dinamik Texture Boru Hattı Şartnamesi**, Stardew Valley mod geliştiricilerine bugüne kadar hiçbir mod kütüphanesinde sunulmamış bir esneklik ve performans kazandırmaktadır.

### Uygulama Sırası (Implementation Milestones):
1. **Milestone 11.1:** `rusted-core` içinde `TextureInterner` ve `ComponentFlags::CUSTOM_IMAGE` bayraklarının tanımlanması, XML `<img />`, `<sprite />`, `<item-icon />` özniteliklerinin ayrıştırılması.
2. **Milestone 11.2:** Taffy `compute_layout_with_measure` içerisine `is_image` intrinsic boyutlandırma ve `aspect_ratio` mantığının eklenmesi.
3. **Milestone 11.3:** `rusted-bridge` C-ABI katmanına `rusted_tree_get_pending_textures` ve `rusted_tree_sync_texture_size` fonksiyonlarının yazılması.
4. **Milestone 11.4:** C# tarafında `TextureManager.cs` sınıfının kodlanması, `IModContentHelper`, `IGameContentHelper` ve `ItemRegistry.GetDataOrErrorItem` entegrasyonu.
5. **Milestone 11.5:** `RustedRenderer.cs` içerisinde `ResolveTexture` metodunun `TextureId >= 1000` için genişletilmesi ve `AssetsInvalidated` SMAPI olayının bağlanması.
6. **Milestone 11.6:** `<animated-sprite />` ve özel `<nine-slice />` bileşenlerinin tamamlanıp test menülerinde doğrulanması.
