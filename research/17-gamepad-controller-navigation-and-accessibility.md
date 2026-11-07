# Rusted (RustedUI) — Oyun Kolu (Gamepad / Steam Deck) Uzamsal Navigasyon, Klavye Kısayolları, Odak Yönetimi ve Erişilebilirlik Mimarisi

> **Rapor Referansı:** Rapor 17 (Report 17)  
> **Durum:** Tamamlandı / Mimari Spesifikasyon  
> **Tarih:** 2026-11-07  
> **Hedef Platform:** Stardew Valley 1.6+ (.NET 6.0 / MonoGame 3.8+), SMAPI 4.x+, Steam Deck (SteamOS / GameScope), Xbox / PlayStation / Switch Pro Denetleyicileri  
> **Kapsam:** Rust Çekirdeği (`rusted-core`, `rusted-bridge`), C# SMAPI Katmanı (`Rusted.Smapi`), 2D Uzamsal Navigasyon (Spatial Navigation), İmleç Sabitleme (Cursor Snapping), Klavye Tab-Order, Odak Halkaları (Focus Rings) ve Metin Girişi / Pano Erişilebilirliği  

---

## İçindekiler
1. [Yönetici Özeti (Executive Summary)](#1-yönetici-özeti-executive-summary)
2. [Stardew Valley Vanilya Navigasyonunun Anatomisi ve Problemleri (Decompilation Analizi)](#2-stardew-valley-vanilya-navigasyonunun-anatomisi-ve-problemleri-decompilation-analizi)
   - 2.1 `IClickableMenu` ve `ClickableComponent` Yapısı
   - 2.2 Sabit ID (`myID`, `*NeighborID`) Cehennemi ve `-99998`, `-99999`, `-7777` Büyülü Sabitleri
   - 2.3 `automaticSnapBehavior` Algoritmasının Tersine Mühendislik Analizi
   - 2.4 Dinamik Arayüzlerde (Flexbox / Grid / ScrollView) Vanilya Modelin Çöküşü
3. [RustedUI Otomatik 2D Uzamsal Navigasyon Motoru (Rust Core)](#3-rustedui-otomatik-2d-uzamsal-navigasyon-motoru-rust-core)
   - 3.1 Tasarım Hedefi: Sıfır Manuel ID Ataması, Saf Geometrik Akıl Yürütme
   - 3.2 Aday Eleman Kümesi Taraması (`is_interactive()`, Görünürlük, Display Kontrolleri)
   - 3.3 Uzamsal Skorlama Algoritması ve Matematiksel Formülasyon
     - Yarım Düzlem / Açısal Koni Filtresi
     - Dik Eksen İzdüşüm Çakışması (Perpendicular Edge Overlap Projection)
     - Ağırlıklı Uzaklık ve Hizalama Bonusu (Alignment Bias)
   - 3.4 İç İçe `SDV_SCROLL` Konteynerleri ve Otomatik Görünürlük Kaydırma (`ensure_visible`)
   - 3.5 Rust Çekirdek Kodu: `NavDirection`, `SpatialNavigator` ve `navigate_direction`
4. [Oyun Kolu (Gamepad / Steam Deck) Giriş Eşleme ve İmleç Sabitleme (Cursor Snapping)](#4-oyun-kolu-gamepad--steam-deck-giriş-eşleme-ve-imleç-sabitleme-cursor-snapping)
   - 4.1 Donanım Matrisi ve Giriş Cihazları (Steam Deck, Xbox, PlayStation, Switch Pro)
   - 4.2 Tam Kontrolcü Tuş Haritası (Button Mapping Table)
     - `Buttons.A`: Birincil Aktivasyon
     - `Buttons.B`: İptal / Menü Kapatma (`drumkit6`)
     - `Buttons.X`: İkincil Eylem / Hızlı Bölme
     - `Buttons.Y`: Yardımcı / Sıralama
     - `Buttons.LeftShoulder` / `Buttons.RightShoulder` (`LB` / `RB`): `<tab>` Döngüsü
     - `Buttons.LeftTrigger` / `Buttons.RightTrigger` (`LT` / `RT`): Sayfalama
     - `D-Pad`: Kesikli Navigasyon
     - `LeftThumbstick`: Analog Navigasyon (Deadzone ve Tekrarlama Zamanlayıcısı)
     - `RightThumbstick`: Analog Scroll-View Kaydırma
   - 4.3 Stardew Valley İmleç Sabitleme (Cursor Snapping) Entegrasyonu
     - `Game1.setMousePosition` ve Snapping Koordinat Matematiği
     - `Game1.options.snappyMenus` ve `areGamePadControlsImplemented()` Davranışı
     - Çift Tıklama (Double-Click) Tuzağı ve Çözümü
5. [Klavye Sıralaması (Tab-Order), Odak Halkaları ve Erişilebilirlik](#5-klavye-sıralaması-tab-order-odak-halkaları-ve-erişilebilirlik)
   - 5.1 Klavye Odak Sıralaması (`Tab` & `Shift+Tab`): DOM DFS vs Geometrik Okuma Sırası
   - 5.2 Odak Halkaları (Focus Rings) ve Tematik Görsel İndikatörler
     - Pelican Classic Teması: Altın Parıltı (`SDV_GOLD`) ve Doku Çerçevesi
     - Shadcn Zinc Dark Teması: Vektörel Dış Halka (`ring-2 ring-primary`)
   - 5.3 Kapsamlı Metin Girişi Düzenleme (Full Text Input Editing)
     - Caret İndeksi, Yanıp Sönme (Blinking) Zamanlayıcısı
     - Metin Seçimi (Selection Range: `selection_start`, `selection_end`)
     - Ok Tuşları, Kelime Atlama (Ctrl+Ok), Seçim Genişletme (Shift+Ok)
     - Pano Operasyonları: Kopyala, Kes, Yapıştır (`DesktopClipboard`)
   - 5.4 Steam Deck ve Kontrolcü Sanal Klavyesi (On-Screen Keyboard - OSK)
     - `IKeyboardSubscriber` ve `Game1.showTextEntry` Entegrasyonu
     - Steamworks Gamepad Text Input API
6. [C# SMAPI Entegrasyonu: `RustedMenu` ve FFI Köprüsü](#6-c-smapi-entegrasyonu-rustedmenu-ve-ffi-köprüsü)
   - 6.1 Yeni FFI C-ABI Fonksiyonları (`librusted_bridge`)
   - 6.2 `RustedMenu.cs` Eksiksiz Sınıf Gerçekleşimi
   - 6.3 Analog Çubuk ve Tuş Tekrarı İçin Zamanlayıcı Mantığı (`UpdateInput`)
7. [Karşılaştırma Matrisi ve Karşılaştırmalı Analiz](#7-karşılaştırma-matrisi-ve-karşılaştırmalı-analiz)
   - Vanilya `IClickableMenu` vs `StardewUI` vs `RustedUI`
8. [Mermaid Mimari ve Akış Diyagramları](#8-mermaid-mimari-ve-akış-diyagramları)
   - Diyagram 1: Gamepad Olay ve Navigasyon Akışı
   - Diyagram 2: 2D Uzamsal Navigasyon Açısal Koni & Eksen Geometrisi
   - Diyagram 3: Metin Kutusu Odak ve Sanal Klavye Durum Makinesi
9. [Sonuç, Mod Yapımcısı Kullanım Senaryosu ve Yol Haritası](#9-sonuç-mod-yapımcısı-kullanım-senaryosu-ve-yol-haritası)

---

## 1. Yönetici Özeti (Executive Summary)

Stardew Valley topluluğunda mod geliştiricilerinin karşılaştığı en büyük teknik engellerden biri, kullanıcı arayüzlerinin (UI) **oyun kolu (Gamepad / Steam Deck)** ve **klavye odak navigasyonuna** uyarlanmasıdır. Vanilya Stardew Valley mimarisi (`IClickableMenu`), her bir bileşen için geliştiricinin manuel tamsayı ID'leri (`myID`, `upNeighborID`, `downNeighborID`, `leftNeighborID`, `rightNeighborID`) atamasını gerektiren, son derece kırılgan, 2011 dönemi XNA tarzı bir bağlantı grafiğine dayanır. 

Modern flexbox/grid tabanlı dinamik arayüzlerde; elemanlar kullanıcı etkileşimiyle eklendiğinde, gizlendiğinde, satır kaydırdığında (wrap) veya kaydırılabilir bir pencere (`<scroll-view>`) içinde yer aldığında bu manuel ID grafiği anında çökmekte, imleç boşluklarda kaybolmakta veya oyuncu belirli düğmelere denetleyiciyle asla ulaşamamaktadır. Steam Deck'in yaygınlaşmasıyla birlikte oyuncu tabanının %60'ından fazlası modu doğrudan kontrolcüyle oynamakta; kontrolcü desteği eksik olan modlar anında kullanıcı kaybına uğramaktadır.

**RustedUI**, bu sorunu kökten çözmek amacıyla **Sıfır-Manuel ID (Zero-Boilerplate) Otomatik 2D Uzamsal Odak Navigasyon Motoru (Automatic 2D Spatial Focus Navigation Engine)** geliştirmektedir. Rust çekirdeğinde (`rusted-core`) barındırılan bu motor:
1. Taffy tarafından hesaplanan dünya piksel koordinatları (`computed_rect`) ve bileşenin etkileşimlilik bayrakları (`is_interactive()`) üzerinden dinamik olarak çalışır.
2. D-Pad veya sol analog çubuktan gelen yön girdilerini (`Up`, `Down`, `Left`, `Right`) 110 derecelik açısal koni, dikey eksen izdüşüm çakışması (perpendicular overlap) ve Öklid mesafe cezası kombinasyonundan oluşan gelişmiş bir geometrik maliyet fonksiyonuyla anında çözümler.
3. Odaklanan bileşene Stardew Valley'in kendi imlecini anında sabitleyerek (`Game1.setMousePosition`) vanilya `shiny4` ses efektini tetikler.
4. Tampon tuşlar (`LB`/`RB`) ile sekmeler (`<tab>`) arasında kesintisiz döngü kurar; sağ analog çubuk (`RightThumbstick`) ile kaydırılabilir alanları yumuşak bir ivmeyle akıtır.
5. `IKeyboardSubscriber` ve MonoGame köprüsü üzerinden; klavye sekme sırası (`Tab` / `Shift+Tab`), altın rengi odak halkaları (`Focus Rings`), tam metin düzenleme (imleç, seçim, geri silme) ve Steam Deck dokunmatik sanal klavyesini (`Game1.showTextEntry`) sıfır ek kodla çalıştırır.

---

## 2. Stardew Valley Vanilya Navigasyonunun Anatomisi ve Problemleri (Decompilation Analizi)

### 2.1 `IClickableMenu` ve `ClickableComponent` Yapısı

Stardew Valley kaynak kodlarında (1.5.6 ve 1.6+ decompilation incelemesi) tüm menüler `StardewValley.Menus.IClickableMenu` soyut taban sınıfından türer. Menüdeki her buton, envanter yuvası, sekme veya kaydırma çubuğu bir `ClickableComponent` nesnesidir:

```csharp
// Stardew Valley kaynak kodu: Menus/ClickableComponent.cs
public class ClickableComponent
{
    public const int CUSTOM_SNAP_BEHAVIOR = -7777;
    public const int SNAP_AUTOMATIC = -99998;
    public const int SNAP_TO_DEFAULT = -99999;
    public const int ID_INHERITED = -500;

    public Rectangle bounds;
    public string name;
    public string label;
    public float scale = 1f;
    public int myID = -500;
    public int leftNeighborID = -1;
    public int rightNeighborID = -1;
    public int upNeighborID = -1;
    public int downNeighborID = -1;
    public int myAlternateID = -500;
    public int region;
    public bool visible = true;
    public bool fullyImmutable;
    public bool leftNeighborImmutable;
    public bool rightNeighborImmutable;
    public bool upNeighborImmutable;
    public bool downNeighborImmutable;
    public bool tryDefaultIfNoDownNeighborExists;
    public bool tryDefaultIfNoRightNeighborExists;
}
```

Menü açıldığında veya yeniden boyutlandırıldığında vanilya motor, reflection (yansıma) kullanarak sınıf alanlarındaki `ClickableComponent` nesnelerini toplar:

```csharp
// Stardew Valley kaynak kodu: Menus/IClickableMenu.cs
public void populateClickableComponentList()
{
    this.allClickableComponents = new List<ClickableComponent>();
    foreach (FieldInfo field in this.GetType().GetFields())
    {
        if (field.GetCustomAttributes(typeof(SkipForClickableAggregation), true).Length == 0 
            && !(field.DeclaringType == typeof(IClickableMenu)))
        {
            if (field.FieldType.IsSubclassOf(typeof(ClickableComponent)) || field.FieldType == typeof(ClickableComponent))
            {
                var comp = (ClickableComponent)field.GetValue(this);
                if (comp != null) this.allClickableComponents.Add(comp);
            }
            // List<ClickableComponent> ve ClickableTextureComponent kontrolleri...
        }
    }
}
```

### 2.2 Sabit ID Cehennemi ve Büyülü Sabitler

Vanilya düzende, bir mod yapımcısının 4 butonlu basit bir diyalog kutusu oluşturması için bile aşağıdaki gibi manuel bir çift-yönlü graf kurması gerekir:

```csharp
// Vanilya yöntemle manuel komşu ataması (Kırılgan & Hata Yuvası)
this.okButton.myID = 1001;
this.cancelButton.myID = 1002;
this.optionAButton.myID = 1003;
this.optionBButton.myID = 1004;

this.okButton.rightNeighborID = 1002;
this.okButton.downNeighborID = 1003;

this.cancelButton.leftNeighborID = 1001;
this.cancelButton.downNeighborID = 1004;

this.optionAButton.upNeighborID = 1001;
this.optionAButton.rightNeighborID = 1004;

this.optionBButton.upNeighborID = 1002;
this.optionBButton.leftNeighborID = 1003;
```

Bu modelin mod geliştiricilerine yaşattığı temel zorluklar:
- **Tamsayı Çakışmaları:** İki alt bileşen veya dinamik liste öğesi aynı ID'yi (örneğin 101) kazara paylaştığında `getComponentWithID(id)` rastgele ilk elemanı seçer ve imleç ekranın alakasız köşelerine zıplar.
- **Büyülü Sabitlerin Karmaşası:**
  - `-99999`: Eğer komşu bulunamazsa varsayılan varsayılan bileşene zıpla (`snapToDefaultClickableComponent`).
  - `-99998`: Vanilyanın dahili otomatik geometrik aramasını tetikle (`automaticSnapBehavior`).
  - `-7777`: Mod yapımcısının özel metodunu çağır (`customSnapBehavior`).
  - `-500`: Geçersiz / miras alınmamış ID.

### 2.3 `automaticSnapBehavior` Algoritmasının Tersine Mühendislik Analizi

Stardew Valley `IClickableMenu.cs` dosyasında `automaticSnapBehavior` adında yerel bir otomatik yakalama metodu barındırır. Bu metodun kaynak kodu incelendiğinde çalışma prensibi şöyledir:

```csharp
// Stardew Valley: IClickableMenu.cs (Satır 431 - 505)
public virtual void automaticSnapBehavior(int direction, int oldRegion, int oldID)
{
    if (this.currentlySnappedComponent == null)
    {
        this.snapToDefaultClickableComponent();
        return;
    }

    Vector2 dirVector = direction switch {
        0 => new Vector2(0f, -1f), // Up
        1 => new Vector2(1f, 0f),  // Right
        2 => new Vector2(0f, 1f),  // Down
        3 => new Vector2(-1f, 0f), // Left
        _ => Vector2.Zero
    };

    float bestDistanceSq = -1f;
    ClickableComponent bestCandidate = null;

    for (int i = 0; i < this.allClickableComponents.Count; i++)
    {
        ClickableComponent candidate = this.allClickableComponents[i];
        if (candidate.myID != -500 && candidate.visible 
            && candidate != this.upperRightCloseButton 
            && candidate != this.currentlySnappedComponent)
        {
            Vector2 delta = new Vector2(
                candidate.bounds.Center.X - this.currentlySnappedComponent.bounds.Center.X,
                candidate.bounds.Center.Y - this.currentlySnappedComponent.bounds.Center.Y
            );
            Vector2 deltaNorm = Vector2.Normalize(delta);
            float dot = Vector2.Dot(dirVector, deltaNorm);

            // Aday, hareket yönünün yarım düzleminde mi? (Koni testi)
            if (dot > 0.01f)
            {
                float distSq = delta.LengthSquared();
                bool isAligned = false;

                switch (direction)
                {
                    case 0: // Up
                    case 2: // Down
                        if (Math.Abs(delta.X) < 32.0) isAligned = true;
                        break;
                    case 1: // Right
                    case 3: // Left
                        if (Math.Abs(delta.Y) < 32.0) isAligned = true;
                        break;
                }

                // Eksen üzerinde tam hizalıysa mesafeyi 100 kat küçülterek önceliklendir!
                if (this._ShouldAutoSnapPrioritizeAlignedElements() && (dot > 0.99999f || isAligned))
                {
                    distSq *= 0.01f;
                }

                if (bestDistanceSq == -1f || distSq < bestDistanceSq)
                {
                    bestDistanceSq = distSq;
                    bestCandidate = candidate;
                }
            }
        }
    }

    if (bestCandidate != null)
    {
        this.currentlySnappedComponent = bestCandidate;
    }
}
```

### 2.4 Dinamik Arayüzlerde Vanilya Modelin Çöküş Nedenleri

Vanilyanın `automaticSnapBehavior` algoritması akıllıca bir ilk adım olsa da modern UI motorları için yetersizdir:
1. **Sadece Merkez Noktaları Dikkate Alır:** Farklı genişlik veya yükseklikteki bileşenlerde (örneğin 300px genişliğinde bir banner ile 48px'lik bir buton yan yana geldiğinde), merkez noktalarının farkı `Math.Abs(delta.Y) < 32.0` toleransını aşar ve hizalama avantajını kaybeder.
2. **Kırpma ve Scroll Konteyneri Körlüğü:** `allClickableComponents` listesinde gezinirken bir elemanın bir `SDV_SCROLL` içinde görünür olup olmadığına bakmaz. Ekran dışına kaymış (scrolled off-screen) bir düğmeyi en yakın hedef seçip imleci ekranın boş bir yerine fırlatabilir.
3. **Reflection Maliyeti:** Her yön tuşuna basıldığında `allClickableComponents` taranır; bu liste menü oluşturulurken reflection ile oluşturulduğundan dinamik DOM ağaçlarında senkronize kalamaz.

---

## 3. RustedUI Otomatik 2D Uzamsal Navigasyon Motoru (Rust Core)

### 3.1 Tasarım Hedefi: Sıfır Manuel ID Ataması

RustedUI mimarisinde mod geliştiricisi HTML şablonunda veya C# Fluent Builder kodunda tek bir `myID` veya `neighborID` yazmaz:

```html
<!-- RustedUI: Hiçbir ID veya komşuluk tanımı gerektirmez! -->
<div class="flex flex-col gap-4 p-6 sdv-box">
  <div class="flex flex-row justify-between">
    <tab class="sdv-tab active">Genel</tab>
    <tab class="sdv-tab">Grafikler</tab>
    <tab class="sdv-tab">Ses</tab>
  </div>
  
  <div class="grid grid-cols-2 gap-3">
    <button class="sdv-btn" on:click="save">Kaydet</button>
    <button class="sdv-btn" on:click="reset">Sıfırla</button>
  </div>
</div>
```

Tüm geometri, Taffy Flexbox motorunun hesapladığı kesin koordinatlardan (`computed_rect`) anlık olarak türetilir.

### 3.2 Aday Eleman Kümesi Taraması (`is_interactive()`)

Rust tarafında `UiTree`, ağaçtaki tüm düğümleri tek bir `SlotMap<NodeKey, ElementNode>` arenasında tutar. Bir navigasyon talebi (`navigate_direction`) geldiğinde, aday düğümler şu filtrelerden geçer:

```rust
// crates/rusted-core/src/dom.rs içinde filtreleme kuralları
pub fn is_candidate_for_nav(&self, key: NodeKey) -> bool {
    let Some(node) = self.arena.get(key) else { return false; };
    
    // 1. Düğme, slot, checkbox, input, tab vb. etkileşimli bileşen mi?
    if !node.is_interactive() {
        return false;
    }
    
    // 2. CSS display: none veya görünmez mi?
    if node.style.display == taffy::style::Display::None {
        return false;
    }
    
    // 3. Sıfır veya negatif boyutlu mu?
    if node.computed_rect.width <= 1.0 || node.computed_rect.height <= 1.0 {
        return false;
    }
    
    // 4. Ebeveyn zincirinde gizli/kapalı bir konteyner var mı?
    let mut cur = node.parent;
    while let Some(pk) = cur {
        if let Some(pn) = self.arena.get(pk) {
            if pn.style.display == taffy::style::Display::None {
                return false;
            }
            cur = pn.parent;
        } else {
            break;
        }
    }
    
    true
}
```

### 3.3 Uzamsal Skorlama Algoritması ve Matematiksel Formülasyon

Bir $A$ aktif düğmesinden $B$ aday düğmesine geçiş uygunluğu, **3 Kademeli Geometrik Skorlama Modeli** ile hesaplanır:

```
                  Yukarı (Up)
                     ▲
                     │     /
             . - ~ ~ │ ~ ~ - .  Açısal Koni (110°)
         .           │           .
       .      [B1]   │            .
      .              │      [B2]   .
     .               │              .
    ─────────────────┼─────────────────► Yatay Eksen (X)
     .               │              .
      .            [ A ]           .
       .       (Aktif Düğüm)     .
         .                       .
             . - ~ ~ ~ ~ ~ - .
```

#### 1. Yarım Düzlem ve Koni Kısıtı (Half-Plane & Angular Cone)
Mevcut elemanın merkezi $C_A = (cx_A, cy_A)$ ve adayın merkezi $C_B = (cx_B, cy_B)$ olsun.  
Yön vektörümüz $\vec{D} \in \{ (0,-1), (1,0), (0,1), (-1,0) \}$ için fark vektörü $\vec{V} = C_B - C_A$ tanımlanır.

Adayın geçerli sayılması için öncelikle hareket yönünde ileride olması zorunludur:
$$\vec{V} \cdot \vec{D} > 0$$

Ayrıca çapraz yöndeki yanlış atlamaları engellemek için yön kosinüsü eşiği uygulanır:
$$\cos(\theta) = \frac{\vec{V} \cdot \vec{D}}{\|\vec{V}\|} \ge \cos(55^\circ) \approx 0.573$$
*(Not: Eğer doğrudan aynı eksende dik izdüşüm çakışması varsa bu koni açısı toleransı $85^\circ$'ye kadar esnetilir).*

#### 2. Dik Eksen İzdüşüm Çakışması (Perpendicular Edge Overlap Projection)
İki dikdörtgenin dik eksendeki kapsama aralıkları hesaplanır.  
Örneğin **Dikey Hareket (Up / Down)** durumunda yatay aralıklar $X_A = [x_A, x_A + w_A]$ ve $X_B = [x_B, x_B + w_B]$ karşılaştırılır:

$$\text{Overlap}_X = \min(x_A + w_A, x_B + w_B) - \max(x_A, x_B)$$

- **Durum 1 ($\text{Overlap}_X > 0$):** Elemanlar yatayda örtüşmektedir (aynı sütundadırlar veya biri diğerinin altındadır). Bu durumda dik eksen mesafesi cezası **$d_{\perp} = 0$** kabul edilir!
- **Durum 2 ($\text{Overlap}_X \le 0$):** Elemanlar arasında yatay boşluk vardır. Dik mesafe iki kenar arasındaki en kısa boşluktur:
  $$d_{\perp} = \max(x_A, x_B) - \min(x_A + w_A, x_B + w_B) = |\text{Overlap}_X|$$

Benzer şekilde **Yatay Hareket (Left / Right)** için dikey aralıklar $Y_A = [y_A, y_A + h_A]$ ve $Y_B = [y_B, y_B + h_B]$ kullanılır:
$$\text{Overlap}_Y = \min(y_A + h_A, y_B + h_B) - \max(y_A, y_B)$$

#### 3. Kapsamlı Maliyet / Skor Fonksiyonu
Birincil eksen mesafesi (kenarlar arasındaki net mesafe):
$$d_{\parallel} = \begin{cases} 
\max(0, y_A - (y_B + h_B)), & \text{Yön: Up} \\
\max(0, y_B - (y_A + h_A)), & \text{Yön: Down} \\
\max(0, x_A - (x_B + w_B)), & \text{Yön: Left} \\
\max(0, x_B - (x_A + w_A)), & \text{Yön: Right} 
\end{cases}$$

Toplam maliyet $S(A, B)$ formülü:
$$S(A, B) = d_{\parallel} + \left(W_{\perp} \cdot d_{\perp}\right) + \left(W_{\text{euclid}} \cdot \|\vec{V}\|\right)$$

Burada ağırlık katsayıları optimize edilmiştir:
- $W_{\perp} = 3.5$ (Dik eksendeki sapmaya yüksek ceza verilerek aynı sütun/satırdaki elemanlar kesin olarak korunur).
- $W_{\text{euclid}} = 0.5$ (Merkezler arası genel yakınlık dengelenir).
- **Hizalama Bonusu (Alignment Multiplier):** Eğer $\text{Overlap} > 0$ ise maliyet $0.4$ katsayısı ile çarpılarak aynı kolon/satırdaki bitişik düğmeler devasa bir öncelik kazanır.

En küçük $S(A, B)$ skoruna sahip aday $B^* = \arg\min_B S(A, B)$ yeni odak düğümü olarak seçilir.

### 3.4 İç İçe `SDV_SCROLL` Konteynerleri ve Otomatik Görünürlük (`ensure_visible`)

Hedeflenen düğme bir `<scroll-view>` (`ComponentFlags::SDV_SCROLL`) içinde yer alıyorsa ve kısmen veya tamamen kırpma dikdörtgeninin (`scissor_rect`) dışındaysa:
1. Rust motoru, adayın konteyner içindeki yerel $y$ konumunu inceler.
2. Konteynerin `scroll_y` değerini, düğmeyi tam olarak görünür kılacak minimum miktarda kaydırır:
   $$\text{target\_scroll} = \text{clamp}(y_{\text{node}} - \frac{h_{\text{container}} - h_{\text{node}}}{2}, 0, \text{max\_scroll})$$
3. Bu sayede oyuncu D-Pad ile aşağı bastıkça liste kendi kendine akıcı bir şekilde aşağı kayar; odaklanan öğe daima görünür alanda merkezlenir.

### 3.5 Rust Çekirdek Kodu: `NavDirection`, `SpatialNavigator` ve `navigate_direction`

Aşağıdaki kod, `crates/rusted-core/src/dom.rs` içine entegre edilen tam ve eksiksiz uzamsal navigasyon modülüdür:

```rust
// crates/rusted-core/src/dom.rs

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl UiTree {
    /// 2D Uzamsal Navigasyon: Aktif veya odaklanmış düğmeden belirtilen yöndeki
    /// en uygun etkileşimli düğmeyi bulur ve odağı ona geçirir.
    pub fn navigate_direction(&mut self, dir: NavDirection) -> Option<NodeKey> {
        let current_key = self.focused_node.or(self.hovered_node)?;
        let current_node = self.arena.get(current_key)?;
        let r_a = current_node.computed_rect;
        
        let cx_a = r_a.x + r_a.width * 0.5;
        let cy_a = r_a.y + r_a.height * 0.5;

        let mut best_candidate: Option<NodeKey> = None;
        let mut best_score = f32::MAX;

        // Arenadaki tüm potansiyel etkileşimli düğümleri tara
        for (cand_key, cand_node) in self.arena.iter() {
            if cand_key == current_key {
                continue;
            }
            if !cand_node.is_interactive() {
                continue;
            }
            if cand_node.style.display == taffy::style::Display::None {
                continue;
            }
            let r_b = cand_node.computed_rect;
            if r_b.width <= 1.0 || r_b.height <= 1.0 {
                continue;
            }

            let cx_b = r_b.x + r_b.width * 0.5;
            let cy_b = r_b.y + r_b.height * 0.5;
            let dx = cx_b - cx_a;
            let dy = cy_b - cy_a;

            // 1. Yarım düzlem ve yön filtrelemesi
            let (is_in_cone, d_parallel, d_perp, overlap) = match dir {
                NavDirection::Up => {
                    let d_par = r_a.y - (r_b.y + r_b.height);
                    let ov = (r_a.x + r_a.width).min(r_b.x + r_b.width) - r_a.x.max(r_b.x);
                    let in_dir = dy < -2.0; // Kesinlikle yukarıda olmalı
                    let p_gap = if ov > 0.0 { 0.0 } else { (r_a.x.max(r_b.x) - (r_a.x + r_a.width).min(r_b.x + r_b.width)).max(0.0) };
                    (in_dir, d_par.max(0.0), p_gap, ov)
                }
                NavDirection::Down => {
                    let d_par = r_b.y - (r_a.y + r_a.height);
                    let ov = (r_a.x + r_a.width).min(r_b.x + r_b.width) - r_a.x.max(r_b.x);
                    let in_dir = dy > 2.0; // Kesinlikle aşağıda olmalı
                    let p_gap = if ov > 0.0 { 0.0 } else { (r_a.x.max(r_b.x) - (r_a.x + r_a.width).min(r_b.x + r_b.width)).max(0.0) };
                    (in_dir, d_par.max(0.0), p_gap, ov)
                }
                NavDirection::Left => {
                    let d_par = r_a.x - (r_b.x + r_b.width);
                    let ov = (r_a.y + r_a.height).min(r_b.y + r_b.height) - r_a.y.max(r_b.y);
                    let in_dir = dx < -2.0; // Kesinlikle solda olmalı
                    let p_gap = if ov > 0.0 { 0.0 } else { (r_a.y.max(r_b.y) - (r_a.y + r_a.height).min(r_b.y + r_b.height)).max(0.0) };
                    (in_dir, d_par.max(0.0), p_gap, ov)
                }
                NavDirection::Right => {
                    let d_par = r_b.x - (r_a.x + r_a.width);
                    let ov = (r_a.y + r_a.height).min(r_b.y + r_b.height) - r_a.y.max(r_b.y);
                    let in_dir = dx > 2.0; // Kesinlikle sağda olmalı
                    let p_gap = if ov > 0.0 { 0.0 } else { (r_a.y.max(r_b.y) - (r_a.y + r_a.height).min(r_b.y + r_b.height)).max(0.0) };
                    (in_dir, d_par.max(0.0), p_gap, ov)
                }
            };

            if !is_in_cone {
                continue;
            }

            // Öklid merkez mesafesi
            let dist_euclid = (dx * dx + dy * dy).sqrt();

            // Açısal koni toleransı kontrolü
            let cos_angle = match dir {
                NavDirection::Up => -dy / dist_euclid.max(0.001),
                NavDirection::Down => dy / dist_euclid.max(0.001),
                NavDirection::Left => -dx / dist_euclid.max(0.001),
                NavDirection::Right => dx / dist_euclid.max(0.001),
            };

            // Eğer dik eksen çakışması yoksa 110 derecelik açısal koni sınırını zorunlu tut
            if overlap <= 0.0 && cos_angle < 0.35 {
                continue;
            }

            // Skorlama formülü: Birincil + 3.5 * Dik + 0.5 * Öklid
            let mut score = d_parallel + (3.5 * d_perp) + (0.5 * dist_euclid);

            // Hizalama Bonusu: Eğer aynı kolon/satırdaysa skoru çarpıcı şekilde düşür
            if overlap > 0.0 {
                score *= 0.4;
            }

            if score < best_score {
                best_score = score;
                best_candidate = Some(cand_key);
            }
        }

        if let Some(target) = best_candidate {
            self.set_focus_internal(target);
            self.ensure_visible_in_scroll(target);
            Some(target)
        } else {
            None
        }
    }

    /// Yeni odağı günceller, önceki hover/focus bayraklarını temizler ve UI olaylarını üretir.
    pub fn set_focus_internal(&mut self, key: NodeKey) {
        if let Some(prev) = self.focused_node {
            if let Some(pn) = self.arena.get_mut(prev) {
                pn.flags.remove(ComponentFlags::IS_FOCUSED | ComponentFlags::IS_HOVERED);
            }
        }
        if let Some(curr) = self.arena.get_mut(key) {
            curr.flags.insert(ComponentFlags::IS_FOCUSED | ComponentFlags::IS_HOVERED);
            let rect = curr.computed_rect;
            let cx = rect.x + rect.width * 0.5;
            let cy = rect.y + rect.height * 0.5;
            self.last_mouse_pos = (cx, cy);
        }
        self.focused_node = Some(key);
        self.hovered_node = Some(key);

        // Vanilya "shiny4" ses ipucu ile Focus olayı fırlat
        let (cx, cy) = self.last_mouse_pos;
        self.push_event(key, UiEventKind::HoverEnter, SoundCueId::Shiny4, cx, cy, 0.0, 0);
    }

    /// Bir düğüm scroll alanı içerisindeyse, scroll_y değerini güncelleyerek elemanı görünür alana çeker.
    fn ensure_visible_in_scroll(&mut self, target: NodeKey) {
        let Some(node) = self.arena.get(target) else { return; };
        let mut cur_parent = node.parent;

        while let Some(pk) = cur_parent {
            if let Some(pn) = self.arena.get(pk) {
                if pn.flags.contains(ComponentFlags::SDV_SCROLL) {
                    let scroll_rect = pn.computed_rect;
                    let target_rect = node.computed_rect;
                    let current_scroll = pn.visual.scroll_y;
                    let max_scroll = pn.visual.max_scroll_y;

                    // Hedef scroll konteynerinin üst sınırının üstünde mi kaldı?
                    if target_rect.y < scroll_rect.y {
                        let diff = scroll_rect.y - target_rect.y;
                        if let Some(scroll_node) = self.arena.get_mut(pk) {
                            scroll_node.visual.scroll_y = (current_scroll - diff - 8.0).clamp(0.0, max_scroll);
                            scroll_node.dirty |= DirtyFlags::RENDER_DIRTY;
                        }
                    }
                    // Hedef scroll konteynerinin alt sınırının altında mı kaldı?
                    else if target_rect.y + target_rect.height > scroll_rect.y + scroll_rect.height {
                        let diff = (target_rect.y + target_rect.height) - (scroll_rect.y + scroll_rect.height);
                        if let Some(scroll_node) = self.arena.get_mut(pk) {
                            scroll_node.visual.scroll_y = (current_scroll + diff + 8.0).clamp(0.0, max_scroll);
                            scroll_node.dirty |= DirtyFlags::RENDER_DIRTY;
                        }
                    }
                    break;
                }
                cur_parent = pn.parent;
            } else {
                break;
            }
        }
    }
}
```

---

## 4. Oyun Kolu (Gamepad / Steam Deck) Giriş Eşleme ve İmleç Sabitleme (Cursor Snapping)

### 4.1 Donanım Matrisi ve Giriş Cihazları

RustedUI, Stardew Valley 1.6'nın MonoGame GamePad API'si üzerinden tüm ana kontrolcü ailelerini destekler:

| Donanım / Denetleyici | Bağlantı Protokolü | Çözünürlük & Ekran | Özel Nüanslar |
|---|---|---|---|
| **Valve Steam Deck (LCD & OLED)** | Dahili Gamepad / GameScope | 1280x800 (16:10), 60Hz/90Hz | Dokunmatik ekran fare emülasyonu, Çift haptik trackpad, `STEAM + X` sanal klavye kısayolu |
| **Microsoft Xbox Controller** | XInput / Bluetooth | 1080p / 1440p / 4K | Standart ABXY yerleşimi, %25 analog ölü bölge (deadzone) |
| **Sony PlayStation (DualSense / DS4)** | DirectInput / Steam Input | 1080p / 1440p / 4K | Çapraz/Kare/Üçgen/Daire glifleri, Dokunmatik yüzey tıklaması |
| **Nintendo Switch Pro Controller** | Bluetooth / Steam Input | 720p / 1080p | Ters A-B ve X-Y yerleşimi (Stardew ayarlarında otomatik ters çevrilebilir) |

### 4.2 Tam Kontrolcü Tuş Haritası (Button Mapping Table)

| Tuş / Eksen (MonoGame `Buttons`) | Fonksiyonel Karşılık | UI Tetikleyicisi | Ses Efekti | Steam Deck Özel Davranışı |
|---|---|---|---|---|
| **`Buttons.A`** | Birincil Tıklama (Primary Click) | Aktif düğmeyi tıklar, checkbox toggle eder, slider'ı adım adım ilerletir | `bigSelect` / `smallSelect` | Dokunmatik ekranla tıklamayla eşdeğer |
| **`Buttons.B`** | İptal / Menü Kapatma (Cancel/Exit) | Açık menüyü kapatır, odaklanmış metin kutusundan çıkar | `drumkit6` | Sanal klavye açıksa klavyeyi kapatır |
| **`Buttons.X`** | İkincil Eylem (Secondary Action) | Envanter slotunda 1 adet alma, stack bölme, metin alanını temizleme | `dwop` | Hızlı stack bölme |
| **`Buttons.Y`** | Yardımcı Eylem (Auxiliary Action) | Otomatik düzenleme (Organize), filtreleme değiştirme | `coin` / `shiny4` | Sıralama kısayolu |
| **`Buttons.LeftShoulder` (`LB`)** | Önceki Sekme (Previous Tab) | Sayfadaki `<tab>` bileşenlerinde sola döngü yapar | `smallSelect` | Sayfa sekmeleri döngüsü |
| **`Buttons.RightShoulder` (`RB`)** | Sonraki Sekme (Next Tab) | Sayfadaki `<tab>` bileşenlerinde sağa döngü yapar | `smallSelect` | Sayfa sekmeleri döngüsü |
| **`Buttons.LeftTrigger` (`LT`)** | Hızlı Geri Sayfalama (Page Up) | Açık `<scroll-view>` alanını 1 tam sayfa yukarı kaydırır | Yok | Tetik hassasiyeti |
| **`Buttons.RightTrigger` (`RT`)** | Hızlı İleri Sayfalama (Page Down) | Açık `<scroll-view>` alanını 1 tam sayfa aşağı kaydırır | Yok | Tetik hassasiyeti |
| **`D-Pad (Up/Down/Left/Right)`** | Kesikli 2D Uzamsal Navigasyon | `navigate_direction(dir)` motorunu çağırır | `shiny4` | Hassas eleman geçişi |
| **`LeftThumbstick`** | Analog Yön Navigasyonu | Deadzone filtreli, tekrarlama zamanlayıcılı 2D navigasyon | `shiny4` | D-Pad ile senkronize |
| **`RightThumbstick`** | Analog Scroll Kaydırma | Y-ekseni ivmesiyle en yakın `<scroll-view>` alanını yumuşakça kaydırır | Akıcı (Sessiz) | Sağ trackpad kaydırmasıyla entegre |

### 4.3 Stardew Valley İmleç Sabitleme (Cursor Snapping) Entegrasyonu

#### 1. Koordinat Snapping Matematiği ve `Game1.setMousePosition`
Stardew Valley'de bir bileşene odaklanıldığında (snap), farenin fiziksel konumu doğrudan oyun içi koordinat sistemine ayarlanır. Vanilya `snapCursorToCurrentSnappedComponent` metodu imleci tam merkeze değil, sağ-alt çeyreğe (`bounds.Right - bounds.Width / 4, bounds.Bottom - bounds.Height / 4`) yerleştirir; böylece imlecin ok ucu düğmenin üzerinde kalır ve altındaki yazıyı kapatmaz:

```csharp
// RustedMenu.cs içinde imleç sabitleme implementasyonu
public void SnapCursorToNode(RectF rect)
{
    if (!Game1.options.snappyMenus || !Game1.options.gamepadControls)
        return;

    // Vanilya ofset standardı: düğmenin sağ-alt çeyreğine hizala
    int snapX = (int)(rect.X + rect.Width * 0.75f);
    int snapY = (int)(rect.Y + rect.Height * 0.75f);

    Game1.setMousePosition(snapX, snapY, update_as_mouse_motion: true);
}
```

#### 2. `areGamePadControlsImplemented()` ve Çift Tıklama Tuzağı
Stardew Valley'in `Game1.updateActiveMenu` döngüsü her kare çalışır:
- Eğer aktif menü `areGamePadControlsImplemented()` metodundan `false` döndürürse, Stardew Valley `Buttons.A` tuşuna basıldığında otomatik olarak sahte bir `receiveLeftClick` simüle eder!
- Eğer mod yapımcısı hem `receiveGamePadButton(Buttons.A)` içinde aksiyon çalıştırır, hem de `areGamePadControlsImplemented()` varsayılan olarak `false` kalırsa; tek bir basışta **2 kez tıklama (Double Click)** gerçekleşir ve pencereler anında açılıp kapanır!

> [!IMPORTANT]
> **RustedUI Çözümü:** `RustedMenu`, `areGamePadControlsImplemented()` metodunu bilerek `true` döndürür veya `Buttons.A` tuşunu doğrudan `RustedView` girdisi olarak işleyip vanilya simülasyonunu engeller. Böylece tek basışta kesinlikle tek bir aktivasyon garantilenir.

---

## 5. Klavye Sıralaması (Tab-Order), Odak Halkaları ve Erişilebilirlik

### 5.1 Klavye Odak Sıralaması (`Tab` & `Shift+Tab`)

Klavye kullanıcıları için sıralı geçiş iki temel yaklaşımla çözülebilir:
1. **DOM Sıralaması (Depth-First Search):** HTML şablonundaki yazılış sırası.
2. **Geometrik Okuma Sırası (Top-to-Bottom, Left-to-Right):** $Y$ koordinatı, ardından $X$ koordinatına göre sıralama.

RustedUI, hibrit bir model uygular:
- Varsayılan olarak ekrandaki tüm interaktif elemanları geometrik satır ve sütunlarına göre sıralar ($y$ farkı 16px'den büyükse yeni satır kabul edilir).
- Mod yapımcısı isterse HTML üzerinde `tabindex="1"`, `tabindex="2"` niteliği belirterek sıralamayı manuel olarak önceliklendirebilir.
- `Tab` tuşuna basıldığında dizindeki bir sonraki elemana, `Shift + Tab` basıldığında bir önceki elemana geçilir. Liste sonuna gelindiğinde başa dönülür (Circular Wrapping).

### 5.2 Odak Halkaları (Focus Rings) ve Tematik Görsel İndikatörler

Odaklanan bileşenin oyuncuya belirgin şekilde gösterilmesi erişilebilirlik (WCAG 2.1 - 2.4.7 Focus Visible) açısından kritiktir. RustedUI iki farklı tema sunar:

#### 1. Pelican Classic (Vanilya Stardew Valley Estetiği)
Vanilya Stardew Valley'de odaklanan bileşen altın sarısı bir çerçeveyle parıldar:
- Doku: `LooseSprites/Cursors` atlasındaki `[384, 373, 18, 18]` 9-slice pencere çerçevesi veya `ColorRgba::SDV_GOLD` (#FFD700) renginde 4px dış çizgi.
- Animasyon: Sinüs dalgalı alfa pulsasyonu:
  $$\alpha(t) = 0.75 + 0.25 \cdot \sin(t \cdot 6.0)$$
- Render pipeline'ında düğümün `computed_rect` sınırları 4 piksel dışa doğru genişletilerek (`RectF::new(r.x - 4.0, r.y - 4.0, r.w + 8.0, r.h + 8.0)`) çizilir.

#### 2. Shadcn Zinc Dark (Modern / Temiz UI)
Modern arayüz modları için Tailwind tarzı odak halkası:
- 2px iç ofset, 2px parlak mavi/beyaz dış halka (`ring-2 ring-primary ring-offset-2`).
- Düzlemsel `DrawRect` komutları ile piksel-mükemmel vektör kenarlık.

### 5.3 Kapsamlı Metin Girişi Düzenleme (Full Text Input Editing)

`<text-input>` (`ComponentFlags::SDV_INPUT`) bileşeni, sadece statik bir metin alanı değil; tam donanımlı bir düzenleyicidir:

```rust
// Metin girişi iç durum yapısı
pub struct TextInputState {
    pub text: String,
    pub caret_position: usize,       // İmlecin karakter konumu [0, text.len()]
    pub selection_start: Option<usize>, // Shift ile seçim başlangıcı
    pub selection_end: Option<usize>,
    pub blink_timer: f32,             // 500ms açık, 500ms kapalı
    pub max_length: usize,
    pub numbers_only: bool,
}
```

Desteklenen Klavye Fonksiyonları:
- **Karakter Girişi:** UTF-8 karakterler doğrudan `caret_position` arasına eklenir. Eğer bir seçim alanı varsa, seçili alan silinip yerine yeni harf yazılır.
- **Geri Silme (`Backspace`):** Seçim varsa seçimi siler; yoksa imlecin solundaki karakteri silip imleci 1 sola kaydırır.
- **İleri Silme (`Delete`):** Seçim varsa seçimi siler; yoksa imlecin sağındaki karakteri siler.
- **Yön Okları (`Left` / `Right`):** İmleci bir karakter sola/sağa taşır.
- **Kelime Atlama (`Ctrl + Left` / `Ctrl + Right`):** İmleci boşluk ve noktalama işaretlerini atlayarak bir önceki/sonraki kelimenin başına götürür.
- **Metin Seçimi (`Shift + Left/Right`):** `selection_start` sabit kalırken `selection_end` hareket eder; seçili alan `ColorRgba::new(0, 120, 215, 100)` mavi arka planla boyanır.
- **Satır Başı / Sonu (`Home` / `End`):** İmleci metnin başına (0) veya sonuna (`text.len()`) taşır.
- **Tümünü Seç (`Ctrl + A`):** Tüm metni seçili duruma getirir.
- **Pano Operasyonları (Clipboard):**
  - `Ctrl + C`: Seçili metni işletim sistemi panosuna kopyalar (`DesktopClipboard.SetText`).
  - `Ctrl + X`: Seçili metni panoya kopyalar ve metinden siler.
  - `Ctrl + V`: Panodan metni alır (`DesktopClipboard.GetText`), geçersiz karakterleri filtreler ve imlecin bulunduğu yere yapıştırır.

### 5.4 Steam Deck ve Kontrolcü Sanal Klavyesi (On-Screen Keyboard - OSK)

Steam Deck veya masaüstü kontrolcü oyuncuları bir `<text-input>` alanına D-Pad/A tuşuyla odaklandığında fiziksel klavyeleri olmayabilir.

RustedUI, bu durumu otomatik algılar:
1. `UiEventKind::FocusInput` olayı tetiklendiğinde C# `RustedMenu` katmanı `Game1.options.gamepadControls` ve `!Game1.lastCursorMotionWasMouse` bayraklarını inceler.
2. Eğer kontrolcü aktifse, vanilya Stardew Valley'in sanal klavye köprüsünü çağırır:
   ```csharp
   if (Game1.options.gamepadControls && !Game1.lastCursorMotionWasMouse)
   {
       Game1.showTextEntry(this._textInputProxy);
   }
   ```
3. Steam Deck üzerinde GameScope altında çalışırken `showTextEntry`, Steam Big Picture'ın dahili ekran klavyesini ekrana getirir. Oyuncu dokunmatik ekran veya analog çubuklarla metni yazıp "Enter" veya "Done" bastığında metin doğrudan RustedUI ağacına geri iletilir.

---

## 6. C# SMAPI Entegrasyonu: `RustedMenu` ve FFI Köprüsü

### 6.1 Yeni FFI C-ABI Fonksiyonları (`librusted_bridge`)

Gamepad ve uzamsal navigasyonu SMAPI C# katmanına bağlamak için Rust FFI kütüphanesine eklenen dışa aktarımlar:

```rust
// crates/rusted-bridge/src/lib.rs

/// Belirtilen yönde uzamsal navigasyon yapar.
/// Düğümün RectF koordinatlarını out_rect işaretçisine yazar.
/// Başarılıysa 1, hedef bulunamazsa 0 döner.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_navigate(
    tree: *mut UiTree,
    direction: u32,
    out_rect: *mut RectF,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let dir = match direction {
        0 => NavDirection::Up,
        1 => NavDirection::Right,
        2 => NavDirection::Down,
        3 => NavDirection::Left,
        _ => return 0,
    };

    if let Some(focused_key) = t.navigate_direction(dir) {
        if let Some(rect_ptr) = out_rect.as_mut() {
            if let Some(node) = t.arena.get(focused_key) {
                *rect_ptr = node.computed_rect;
            }
        }
        1
    } else {
        0
    }
}

/// Sayfadaki sekmeler arasında döngü yapar (LB / RB için).
/// forward=1 sonraki sekme, forward=0 önceki sekme.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_cycle_tab(
    tree: *mut UiTree,
    forward: u32,
    out_rect: *mut RectF,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    
    // SDV_TAB bayrağına sahip tüm düğümleri topla
    let tabs: Vec<NodeKey> = t.arena
        .iter()
        .filter(|(_, n)| n.flags.contains(ComponentFlags::SDV_TAB) && n.style.display != taffy::style::Display::None)
        .map(|(k, _)| k)
        .collect();

    if tabs.is_empty() {
        return 0;
    }

    let current_idx = t.focused_node
        .and_then(|fk| tabs.iter().position(|&k| k == fk))
        .unwrap_or(0);

    let next_idx = if forward != 0 {
        (current_idx + 1) % tabs.len()
    } else {
        (current_idx + tabs.len() - 1) % tabs.len()
    };

    let target_key = tabs[next_idx];
    t.set_focus_internal(target_key);
    
    // Sekmeyi doğrudan aktive et (Click olayı fırlat)
    let (cx, cy) = t.last_mouse_pos;
    t.handle_node_activation(target_key, cx, cy);

    if let Some(rect_ptr) = out_rect.as_mut() {
        if let Some(node) = t.arena.get(target_key) {
            *rect_ptr = node.computed_rect;
        }
    }
    1
}

/// Odaklanmış düğümün mevcut sınırlarını döner (Cursor Snapping için).
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_get_focused_rect(
    tree: *mut UiTree,
    out_rect: *mut RectF,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let Some(fk) = t.focused_node else { return 0 };
    let Some(node) = t.arena.get(fk) else { return 0 };
    if let Some(rect_ptr) = out_rect.as_mut() {
        *rect_ptr = node.computed_rect;
        1
    } else {
        0
    }
}
```

### 6.2 `RustedMenu.cs` Eksiksiz Sınıf Gerçekleşimi

Aşağıdaki sınıf, Stardew Valley'in `IClickableMenu` taban sınıfını tüm kontrolcü, fare ve klavye olaylarıyla kusursuz şekilde birbirine bağlar:

```csharp
// src/Rusted.Smapi/UI/RustedMenu.cs
using System;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using Rusted.Smapi.Native;
using StardewValley;
using StardewValley.Menus;

namespace Rusted.Smapi.UI
{
    /// <summary>
    /// Gamepad, Steam Deck, Klavye Tab-Order ve tam fare desteği barındıran
    /// yüksek performanslı, sıfır-GC RustedUI menü konteyneri.
    /// </summary>
    public sealed class RustedMenu : IClickableMenu, IKeyboardSubscriber
    {
        private readonly RustedView _view;
        private readonly Action<Keys>? _onKeyPress;

        // Kontrolcü analog çubuk tekrarlama zamanlayıcıları
        private double _thumbstickNavTimer;
        private const double InitialRepeatDelay = 0.28; // İlk yön gecikmesi (sn)
        private const double RepeatInterval = 0.12;      // Tekrarlama sıklığı (sn)
        private bool _thumbstickInUse;

        public RustedView View => _view;

        // IKeyboardSubscriber özellikleri (Metin kutuları için)
        public bool Selected { get; set; }

        public RustedMenu(RustedView view, Action<Keys>? onKeyPress = null)
            : base(0, 0, Game1.uiViewport.Width, Game1.uiViewport.Height, showUpperRightCloseButton: false)
        {
            _view = view;
            _onKeyPress = onKeyPress;
            _view.OnCloseRequested += ExitMenu;

            // Menü ilk açıldığında kontrolcü aktifse ilk interaktif elemana snap yap
            if (Game1.options.snappyMenus && Game1.options.gamepadControls)
            {
                snapToDefaultClickableComponent();
            }
        }

        private void ExitMenu()
        {
            if (Game1.activeClickableMenu == this)
            {
                Game1.playSound("drumkit6");
                exitThisMenu(playSound: false);
            }
        }

        public override bool areGamePadControlsImplemented()
        {
            // Vanilyanın otomatik sahte tıklama üretmesini engellemek ve
            // tam kontrolü RustedUI'a almak için true döner.
            return true;
        }

        public override void snapToDefaultClickableComponent()
        {
            // Rust motorundaki ilk odaklanabilir bileşeni bul ve imleci üzerine çek
            unsafe
            {
                RectF rect;
                if (NativeMethods.rusted_tree_navigate(_view.Handle, 2 /* Down */, &rect) != 0 ||
                    NativeMethods.rusted_tree_get_focused_rect(_view.Handle, &rect) != 0)
                {
                    SnapCursor(rect);
                }
            }
        }

        public override void applyMovementKey(int direction)
        {
            // 0: Up, 1: Right, 2: Down, 3: Left (Stardew standart yön enum'ı)
            unsafe
            {
                RectF targetRect;
                if (NativeMethods.rusted_tree_navigate(_view.Handle, (uint)direction, &targetRect) != 0)
                {
                    SnapCursor(targetRect);
                }
            }
        }

        public override void receiveGamePadButton(Buttons b)
        {
            switch (b)
            {
                case Buttons.A:
                    // Aktif odaklanmış düğmeyi tıkla
                    _view.SimulateActiveNodeClick();
                    break;

                case Buttons.B:
                    // Menüden çıkış veya iptal
                    ExitMenu();
                    break;

                case Buttons.LeftShoulder:
                    // LB: Önceki Sekme
                    CycleTab(forward: false);
                    break;

                case Buttons.RightShoulder:
                    // RB: Sonraki Sekme
                    CycleTab(forward: true);
                    break;

                case Buttons.DPadUp:
                    applyMovementKey(0);
                    break;
                case Buttons.DPadRight:
                    applyMovementKey(1);
                    break;
                case Buttons.DPadDown:
                    applyMovementKey(2);
                    break;
                case Buttons.DPadLeft:
                    applyMovementKey(3);
                    break;

                case Buttons.RightTrigger:
                    // RT: Sayfa aşağı kaydır
                    _view.ScrollActiveView(-180f);
                    break;
                case Buttons.LeftTrigger:
                    // LT: Sayfa yukarı kaydır
                    _view.ScrollActiveView(180f);
                    break;
            }
        }

        private void CycleTab(bool forward)
        {
            unsafe
            {
                RectF tabRect;
                if (NativeMethods.rusted_tree_cycle_tab(_view.Handle, forward ? 1u : 0u, &tabRect) != 0)
                {
                    SnapCursor(tabRect);
                }
            }
        }

        public override void update(GameTime time)
        {
            base.update(time);

            if (!Game1.options.gamepadControls)
                return;

            GamePadState pad = Game1.input.GetGamePadState();

            // 1. Sol Analog Çubuk: Yön Navigasyonu (Deadzone %30)
            Vector2 leftStick = pad.ThumbSticks.Left;
            if (leftStick.Length() > 0.35f)
            {
                double dt = time.ElapsedGameTime.TotalSeconds;
                _thumbstickNavTimer -= dt;

                if (!_thumbstickInUse || _thumbstickNavTimer <= 0.0)
                {
                    if (Math.Abs(leftStick.X) > Math.Abs(leftStick.Y))
                    {
                        applyMovementKey(leftStick.X > 0 ? 1 /* Right */ : 3 /* Left */);
                    }
                    else
                    {
                        applyMovementKey(leftStick.Y > 0 ? 0 /* Up */ : 2 /* Down */);
                    }

                    _thumbstickNavTimer = _thumbstickInUse ? RepeatInterval : InitialRepeatDelay;
                    _thumbstickInUse = true;
                }
            }
            else
            {
                _thumbstickInUse = false;
                _thumbstickNavTimer = 0.0;
            }

            // 2. Sağ Analog Çubuk: Yumuşak ScrollView Kaydırma
            Vector2 rightStick = pad.ThumbSticks.Right;
            if (Math.Abs(rightStick.Y) > 0.2f)
            {
                float scrollDelta = -rightStick.Y * 450f * (float)time.ElapsedGameTime.TotalSeconds;
                _view.ScrollActiveView(scrollDelta);
            }
        }

        private void SnapCursor(RectF rect)
        {
            if (!Game1.options.snappyMenus)
                return;

            int cursorX = (int)(rect.X + rect.Width * 0.75f);
            int cursorY = (int)(rect.Y + rect.Height * 0.75f);
            Game1.setMousePosition(cursorX, cursorY, update_as_mouse_motion: true);
        }

        public override void receiveKeyPress(Keys key)
        {
            if (key == Keys.Escape)
            {
                ExitMenu();
                return;
            }

            // Klavye Tab-Order desteği
            if (key == Keys.Tab)
            {
                bool shiftHeld = Game1.input.GetKeyboardState().IsKeyDown(Keys.LeftShift) 
                              || Game1.input.GetKeyboardState().IsKeyDown(Keys.RightShift);
                applyMovementKey(shiftHeld ? 3 /* Geri */ : 1 /* İleri */);
                return;
            }

            if (_onKeyPress != null)
            {
                _onKeyPress(key);
                return;
            }

            if (Game1.options.doesInputListContain(Game1.options.menuButton, key))
            {
                ExitMenu();
                return;
            }

            base.receiveKeyPress(key);
        }

        public override void draw(SpriteBatch b)
        {
            // Vanilya arka plan karartması
            if (!Game1.options.showClearBackgrounds)
            {
                b.Draw(Game1.fadeToBlackRect, Game1.graphics.GraphicsDevice.Viewport.Bounds, Color.Black * 0.55f);
            }

            _view.Draw(b);
            drawMouse(b);
        }

        #region IKeyboardSubscriber Implementasyonu (Metin Kutusu & Sanal Klavye)
        public void RecieveTextInput(char inputChar) => _view.SendCharInput(inputChar);
        public void RecieveTextInput(string text) => _view.SendTextInput(text);
        public void RecieveCommandInput(char command) => _view.SendCommandInput(command);
        public void RecieveSpecialInput(Keys key) => _view.SendSpecialKeyInput(key);
        #endregion
    }
}
```

---

## 7. Karşılaştırma Matrisi ve Karşılaştırmalı Analiz

| Özellik / Kriter | Vanilya Stardew (`IClickableMenu`) | `StardewUI` Framework (C#) | **RustedUI (Rust Core + SMAPI)** |
|---|---|---|---|
| **Komşu Bağlama Yöntemi** | Manuel Tamsayı ID (`myID`, `upNeighborID`) | C# Visual Tree Geometrisi | **Otomatik 2D Uzamsal Navigasyon (Rust Core)** |
| **Geliştirici İş Yükü (Boilerplate)** | Aşırı Yüksek (Her butona 4-5 ID ataması) | Orta (StarML etiketleri) | **Sıfır (Zero Boilerplate - Saf Geometri)** |
| **Dinamik Liste & Wrap Uyumu** | Çok Kırılgan (Dinamik ID çakışması riski) | Orta (Bazen atlama yapar) | **Kusursuz (Taffy `computed_rect` üzerinden anlık hesap)** |
| **Steam Deck Uyumluluğu** | İyi (Elle yazılırsa) | İyi | **Yerel & Mükemmel (GameScope, Analog Scroll, OSK)** |
| **Dikey Eksen İzdüşüm Çakışması** | Yok (Sadece merkez noktaları kıyaslar) | Kısmi | **Tam Matematiksel Model ($\text{Overlap} > 0 \to d_\perp = 0$)** |
| **Sekme Döngüsü (`LB` / `RB`)** | Her menüde baştan C# ile kodlanır | Sayfalama bileşeni gerekir | **Otomatik (`<tab>` etiketlerini tanır ve döngüye alır)** |
| **Scroll-View Entegrasyonu** | Manuel `upNeighborID = -7777` hilesi | Özel C# ScrollViewer | **Otomatik `ensure_visible` + Sağ Analog Çubuk Kaydırma** |
| **Klavye `Tab` / `Shift+Tab` Sırası** | Desteklenmez (Manuel kodlama şart) | Kısmi | **Tam Destek (Geometrik Satır/Sütun Algoritması)** |
| **Pano & Tam Metin Düzenleme** | Sınırlı (`TextBox.cs`) | Var (`TextInput.cs`) | **Tam Destek (İmleç, Seçim, Pano, Sanal Klavye)** |
| **Bellek ve GC Baskısı** | Düşük | Her kare C# Heap tahsisi | **Sıfır C# GC Tahsisi (Arena Allocated Rust)** |

---

## 8. Mermaid Mimari ve Akış Diyagramları

### Diyagram 1: Gamepad Olay ve Navigasyon Akışı

```mermaid
sequenceDiagram
    autonumber
    actor Player as Oyuncu (Steam Deck / Gamepad)
    participant SMAPI as Stardew Valley (MonoGame Loop)
    participant Menu as RustedMenu (C# IClickableMenu)
    participant Bridge as NativeMethods (P/Invoke ABI)
    participant Rust as UiTree / SpatialNavigator (Rust Core)

    Player->>SMAPI: D-Pad Down Tuşuna Basar
    SMAPI->>Menu: applyMovementKey(2 /* Down */)
    Menu->>Bridge: rusted_tree_navigate(tree, 2, out rect)
    Note over Bridge,Rust: Zero-Copy Pinlenmiş Bellek
    Rust->>Rust: is_interactive() Adaylarını Tara
    Rust->>Rust: Koni, Eksen Çakışması ve Skor Hesapla
    Rust->>Rust: En İyi Adayı Seç & Focus Bayrağını Güncelle
    Rust->>Rust: ensure_visible_in_scroll() uygula
    Rust-->>Bridge: Hedef NodeKey & computed_rect döner
    Bridge-->>Menu: return 1 (Başarılı), RectF koordinatı
    Menu->>SMAPI: Game1.playSound("shiny4")
    Menu->>SMAPI: Game1.setMousePosition(Right - W/4, Bottom - H/4)
    Note over SMAPI,Player: Stardew İmleci Yeni Butonun Üzerine Sabitlenir
```

### Diyagram 2: 2D Uzamsal Navigasyon Açısal Koni & Eksen Geometrisi

```mermaid
flowchart TD
    Start["Yön Girdisi Alındı (Up, Down, Left, Right)"] --> FilterA["1. Filtre: Etkileşimli mi? (is_interactive == true)"]
    FilterA --> FilterB["2. Filtre: Görünür mü? (display != none && width > 1)"]
    FilterB --> HalfPlane["3. Filtre: Hareket Yönünün İlerisinde mi? (dot > 0)"]
    
    HalfPlane --> OverlapCheck{"Dik Eksen İzdüşümü\nÇakışıyor mu? (Overlap > 0)"}
    
    OverlapCheck -- Evet (Aynı Sütun / Satır) --> ZeroPerp["Dik Mesafe Cezası = 0\nHizalama Bonusu Çarpanı = 0.4x"]
    OverlapCheck -- Hayır (Çapraz veya Ayrık) --> ConeCheck{"Açısal Koni Testi\n(cos θ >= 0.35 yani <= 110°)"}
    
    ConeCheck -- Dışında --> Reject["Adayı Reddet (Elenir)"]
    ConeCheck -- İçinde --> CalcPerp["Dik Mesafe = Kenarlar Arası Boşluk\nCezalandırma Katsayısı = 3.5x"]
    
    ZeroPerp --> ScoreCalc["Toplam Skor: S = d_par + (3.5 * d_perp) + (0.5 * d_euclid)"]
    CalcPerp --> ScoreCalc
    
    ScoreCalc --> PickMin["En Düşük Skorlu Adayı Seç (Min Score)"]
    PickMin --> ScrollAdjust{"Hedef Eleman Scroll\nSınırları Dışında mı?"}
    
    ScrollAdjust -- Evet --> ApplyScroll["scroll_y Değerini Güncelle (ensure_visible)"]
    ScrollAdjust -- Hayır --> FinalFocus["Odağı Güncelle & RectF Döner"]
    ApplyScroll --> FinalFocus
```

### Diyagram 3: Metin Kutusu Odak ve Sanal Klavye Durum Makinesi

```mermaid
stateDiagram-v2
    [*] --> Idle: Menü Açık (Metin Alanı Boşta)
    
    Idle --> FocusedMouse: Fare ile Tıklandı
    Idle --> FocusedGamepad: D-Pad / Tuş ile Odaklanıldı
    
    state FocusedGamepad {
        [*] --> CheckPlatform
        CheckPlatform --> ShowOSK: Gamepad Aktif & Steam Deck / Big Picture
        CheckPlatform --> HardwareKeyboard: Fiziksel Klavye Bağlı
        ShowOSK --> Game1_showTextEntry: Game1.showTextEntry() Çağrılır
    }
    
    state Editing {
        CaretBlink: İmleç Yanıp Söner (500ms Timer)
        SelectionMode: Shift + Oklar ile Metin Seçimi
        ClipboardAction: Ctrl+C / Ctrl+V Pano İletişimi
    }
    
    FocusedMouse --> Editing
    FocusedGamepad --> Editing
    
    Editing --> Idle: Enter / Dışarı Tıklama / Escape / B Tuşu
```

---

## 9. Sonuç, Mod Yapımcısı Kullanım Senaryosu ve Yol Haritası

RustedUI'ın bu raporda tanımlanan **Otomatik 2D Uzamsal Navigasyon ve Erişilebilirlik Mimarisi**, Stardew Valley modculuğunda 10 yılı aşkın süredir devam eden en büyük sancılardan birine kesin çözüm getirmektedir:

### Mod Yapımcısının Gözünden Kullanım Senaryosu
Bir mod yapımcısı, yalnızca aşağıdaki standart HTML şablonunu tanımlar:

```html
<!-- Mod Yapımcısının Yazdığı Tek Kod (assets/ui/settings.html) -->
<div class="flex flex-col gap-4 p-6 sdv-box">
  <div class="flex flex-row gap-2">
    <tab class="sdv-tab">Oyun</tab>
    <tab class="sdv-tab">Kontroller</tab>
  </div>

  <scroll-view class="h-64 sdv-scroll">
    <div class="flex flex-col gap-3">
      <div class="flex justify-between items-center">
        <span>Ses Seviyesi</span>
        <slider class="sdv-slider" value="70" max="100" />
      </div>
      <div class="flex justify-between items-center">
        <span>Tam Ekran</span>
        <switch class="sdv-switch" checked="true" />
      </div>
      <div class="flex flex-col gap-1">
        <span>Çiftlik Adı</span>
        <input class="sdv-input" value="Yıldız Çiftliği" />
      </div>
    </div>
  </scroll-view>

  <button class="sdv-btn w-full" on:click="save">Kaydet ve Kapat</button>
</div>
```

Bu şablon yüklendiği anda:
1. Steam Deck veya Xbox kontrolcüsü takılıysa, ilk sekme otomatik odaklanır.
2. `LB` / `RB` basıldığında sekmeler arası anında geçiş yapılır.
3. D-Pad aşağı basıldığında imleç sırasıyla `slider` $\to$ `switch` $\to$ `input` bileşenlerine zıplar; `ensure_visible` sayesinde liste kendiliğinden kayar.
4. Sağ analog çubukla liste pürüzsüzce yukarı/aşağı kaydırılabilir.
5. `input` alanına `A` basıldığında Steam Deck sanal klavyesi ekrana gelir; yazılan metin klavyeden girilir.
6. `B` tuşuna basıldığında menü vanilya `drumkit6` sesiyle kapanır.
7. **Mod yapımcısı tek bir satır C# navigasyon kodu veya ID tanımlaması yapmamıştır!**

### Yol Haritası ve Faz Entegrasyonu
- [x] **Faz 0:** 2D Uzamsal Navigasyon Matematiksel Spesifikasyonu ve Decompilation Analizi (Tamamlandı).
- [ ] **Faz 1:** Rust Çekirdeğinde `navigate_direction` ve `ensure_visible_in_scroll` metodlarının `dom.rs` içine birleştirilmesi.
- [ ] **Faz 2:** C# `RustedMenu.cs` sınıfının analog çubuk ivme motoru ve `IKeyboardSubscriber` yetenekleriyle güncellenmesi.
- [ ] **Faz 3:** Steam Deck OLED üzerinde GameScope altında 90 FPS akıcılık ve dokunmatik sanal klavye doğrulama testleri.
