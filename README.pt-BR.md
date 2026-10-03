<div align="center">

<img src="NotchBuddy/Assets.xcassets/AppIcon.appiconset/icon_256x256.png" width="96" alt="Ícone do IA MOTION">

# IA MOTION

**Um pequeno amigo que vive no notch do seu Mac — ou no topo da tela no Windows e Linux — e fica de olho nas sessões dos seus agentes de IA de programação.**

Aprove permissões, assista seus agentes trabalharem, solte um arquivo, converse com o Claude — tudo sem sair do que você está fazendo.

![macOS 15+](https://img.shields.io/badge/macOS-15%2B-black?logo=apple)
![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-AppImage%20%7C%20deb%20%7C%20rpm-FCC624?logo=linux&logoColor=black)
![Swift 6](https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white)
![SwiftUI](https://img.shields.io/badge/SwiftUI-native-0A84FF)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/Louis-CFM/IA MOTION?style=social)

<img src="docs/media/demo.gif" width="760" alt="IA MOTION em ação">

</div>

---

## Por que

Alguns estúdios mostraram companheiros de notch lindos… e nunca deixaram ninguém usá-los.
**IA MOTION é a versão aberta.** Cada linha de código, cada animação, cada som — livre para usar, ler, fazer fork e remixar.

Conheça **Mochi**: um squircle (quadrado arredondado) macio com olhos grandes que sai do seu notch, acena dizendo olá, acompanha seu cursor com os olhos, fica irritado quando você o cutuca (e tonto se você insistir), e te avisa no momento em que o Claude Code precisa de você.

## Funcionalidades

- 🤖 **Claude Code, Cursor, Codex, Gemini CLI, Antigravity e outros agentes, ao vivo** — veja cada sessão no seu notch: o que ele lê, edita e executa, passo a passo. Marque um payload de hook com `IA MOTION_agent` para dar a qualquer agente sua própria pílula (veja [`docs/AGENTS.md`](docs/AGENTS.md)). Terminou? Mochi dá um pequeno pulo de alegria.
- Veja o que o Claude está editando, ao vivo no notch: cada modificação de arquivo mostra o nome do arquivo e as contagens +N −M no ticker, toque para ler o diff completo.
- ✅ **Aprove e responda a partir do notch** — pedidos de permissão do Claude Code aparecem com **Allow / Deny / Always**; os prompts `AskUserQuestion` mostram as escolhas direto no notch (seleção única ou múltipla, até 4 perguntas). Um clique, ou "Reply in terminal" para voltar ao CLI. O Codex também tem Allow / Deny.
- 🧑‍💻 **Pule para o terminal certo** — abra a janela exata do terminal de uma sessão *(macOS)*.
- 💬 **Converse com Claude, Gemini, OpenAI ou um modelo local (Ollama / LM Studio)** — clique no nome do modelo acima da caixa de chat para trocar de provedor e escolher um modelo. Provedores em nuvem usam sua própria chave de API; provedores locais se conectam a um servidor rodando no seu Mac. *(Gemini, OpenAI e modelos locais: macOS)*
- 📊 **Uso do plano Claude** *(macOS, build do GitHub)* — uma pequena pílula no cabeçalho do notch mostra os limites do seu plano Claude para 5 horas e semanal. Ative isso em Settings → Agents → Plan usage. Apenas planos Pro e Max.
- 📋 **Declare as ferramentas que você usa** — abra Settings → Active pills e escolha sua ferramenta de workspace principal (VS Code, Cursor, Codex ou Antigravity), depois ative até mais 4: Gemini CLI, Anthropic, Google AI, OpenAI, Ollama, LM Studio e integrações de serviços *(macOS)*.
- 📎 **Solte um arquivo no notch** — Mochi se transforma em uma caixa e o engole, depois faça uma pergunta sobre ele ou envie por email *(email: macOS, Mail.app)*.
- 🪟 **Arraste o Mochi para qualquer janela** — anexe essa janela como contexto para o Claude *(macOS)*.
- 🔌 **Integrações** — Pagamentos do Stripe, fluxos de trabalho do n8n, GitHub (PRs abertos, revisões solicitadas, status de CI), deployments na Vercel, emails via Resend, Notion, Cal.com. Cada um ganha seu próprio Mochi colorido.
- 🎵 **Pílula do Apple Music** *(macOS, build do GitHub)* — adicione a pílula do Apple Music em Settings → Active pills para ver o que está tocando e controlar a reprodução pelo notch; Mochi dança enquanto toca.
- 🎭 **Um personagem de verdade** — respiração inativa, piscadas, olhos numa esfera que seguem o mouse, emotes, 28 sons feitos à mão, uma saudação ao iniciar.
- 🫥 **Invisível quando inativo** — se esconde quando nada está rodando, espreita quando você passa o mouse no notch (a borda superior da tela no Windows e Linux).
- 🖥️ **Qualquer Mac, com notch ou não** — em um iMac, um Mac mini, ou um MacBook com a tampa fechada num display externo, Mochi fica em uma pequena barra no topo da tela.
- 🔒 **Privado por design** — sem telemetria, sem conta. As chaves vivem no seu Keychain do macOS, Windows Credential Manager ou Linux Secret Service (GNOME Keyring, KWallet). O app só fala com os serviços que você conecta.

<table>
<tr>
<td><img src="docs/media/claude-code.png" alt="Sessão do Claude Code"></td>
<td><img src="docs/media/stripe.png" alt="Pagamentos no Stripe"></td>
</tr>
<tr>
<td><img src="docs/media/chat.png" alt="Chat com Claude"></td>
<td><img src="docs/media/dizzy.png" alt="Muitas cutucadas"></td>
</tr>
</table>

## Instalação

### Download para macOS

1. Pegue o `IA MOTION.zip` mais recente em [Releases](https://github.com/Louis-CFM/IA MOTION/releases).
2. Descompacte e mova o **IA MOTION.app** para `/Applications`.
3. Inicie-o e clique em **Open** quando o macOS pedir para confirmar. Atualizando da versão 0.1.0? O macOS pode pedir, uma vez para cada chave que você salvou, para deixar o IA MOTION usá-la: insira a senha do seu Mac e clique em **Always Allow**.

### Windows

O instalador do Windows está **temporariamente indisponível**. O Microsoft Defender marca erroneamente o instalador não assinado como malware; um relato de falso-positivo está sob revisão na Microsoft e o instalador voltará assim que for liberado e assinado.
Até lá, você pode [compilá-lo do código-fonte](#compilar-do-código-fonte).

Não há notch em um PC, então a ilha desliza da borda superior da tela em vez de se esconder dentro de um. Veja [`windows/README.md`](windows/README.md) para o resto das diferenças.

### Linux

A primeira build para Linux foi lançada como beta: baixe em [IA MOTION para Linux 0.1.1 (beta)](https://github.com/Louis-CFM/IA MOTION/releases/tag/linux-v0.1.1), apenas x86_64 por enquanto. Versões posteriores estarão em [Releases](https://github.com/Louis-CFM/IA MOTION/releases) nas tags `linux-v*`.

- **AppImage** (qualquer distribuição): `chmod +x IA MOTION-Linux-*.AppImage`, e então execute-o.
- **Debian / Ubuntu**: `sudo apt install ./IA MOTION-Linux-*.deb`
- **Fedora / openSUSE**: `sudo dnf install ./IA MOTION-Linux-*.rpm`

Verifique um download com `sha256sum -c SHA256SUMS --ignore-missing`. Chat com Gemini CLI, Antigravity, Google AI, OpenAI e modelo local (Ollama / LM Studio) são apenas para macOS por enquanto.

A ilha fica na borda superior em compositores com layer-shell — COSMIC, KDE Plasma, Hyprland, Sway e outros compositores wlroots. O GNOME não tem layer-shell, então nele ele abre como uma janela comum. Veja [`windows/README.md`](windows/README.md#linux).

### Compilar do código-fonte

**macOS** — requisitos: macOS 15+, Xcode 16+, [XcodeGen](https://github.com/yonaskolb/XcodeGen).

```bash
brew install xcodegen
git clone https://github.com/Louis-CFM/IA MOTION.git
cd IA MOTION/NotchBuddy
xcodegen
open NotchBuddy.xcodeproj   # e então ⌘R
```

**Windows** — requisitos: [Rust](https://rustup.rs), Node 20+, MSVC build tools.

```powershell
git clone https://github.com/Louis-CFM/IA MOTION.git
cd IA MOTION/windows
npm install
npm run pack                # o instalador vai parar em windows/release/
```

**Linux** — requisitos: [Rust](https://rustup.rs), Node 20+, e os pacotes de desenvolvimento WebKitGTK, gtk-layer-shell e appindicator (nomes de Debian/Ubuntu abaixo).

```bash
sudo apt install build-essential pkg-config \
  libwebkit2gtk-4.1-dev libgtk-layer-shell-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev libdbus-1-dev patchelf \
  gstreamer1.0-plugins-base gstreamer1.0-plugins-good
git clone https://github.com/Louis-CFM/IA MOTION.git
cd IA MOTION/windows
npm install
npm run pack                # AppImage, .deb e .rpm vão parar em windows/release/
```

## Configuração

Clique no ícone do IA MOTION na barra de menu (macOS) ou na bandeja do sistema (Windows, Linux) → **Settings…**

| O que | Por que | Onde a chave vai |
|---|---|---|
| **Hooks do Claude Code** | sessões ao vivo e aprovações | **Install hooks** — IA MOTION faz backup de `~/.claude/settings.json`, mescla seus hooks e mostra o diff antes de escrever qualquer coisa |
| **Plano do Claude** *(macOS, build do GitHub)* | Medidor de uso do plano no cabeçalho do notch | **Install relay** em Settings → Agents → Plan usage, depois ative "Show in the notch" |
| **Hooks do Gemini CLI** *(macOS)* | Sessões do Gemini CLI na ilha | **Install hooks** em Settings → Gemini CLI — faz backup de `~/.gemini/settings.json` |
| **Hooks do Antigravity (agy)** *(macOS)* | Sessões do agy na ilha | **Install hooks** em Settings → Antigravity — faz backup de `~/.gemini/config/hooks.json` |
| **Chave de API da Anthropic** | chat e perguntas sobre arquivos | Settings → Anthropic API · Keychain / Windows Credential Manager / Secret Service |
| **Chave de API do Google AI** *(macOS)* | chat com Google AI (Gemini) | Settings → Chat — other providers · Keychain |
| **Chave de API da OpenAI** *(macOS)* | chat com OpenAI | Settings → Chat — other providers · Keychain |
| **Servidor Ollama** *(macOS)* | chat com modelos locais via Ollama | Settings → Chat → Local models → **Connect** |
| **Servidor LM Studio** *(macOS)* | chat com modelos locais via LM Studio | Settings → Chat → Local models → **Connect** |
| **Pílulas ativas** *(macOS)* | escolha quais ferramentas e agentes aparecem na ilha | Settings → Active pills |
| Stripe, n8n, GitHub, Vercel, Resend, Notion, Cal.com | as pílulas de serviços | Keychain / Windows Credential Manager / Secret Service, todas opcionais |

Se o IA MOTION não estiver rodando, o hook encerra imediatamente: **o Claude Code nunca é bloqueado.**

## Coisas para testar

| Faça isso | Mochi faz aquilo |
|---|---|
| Passe o mouse no notch (borda superior no Windows/Linux) | espreita e diz oi 👋 |
| Clique nele | abre |
| Passe o mouse no Mochi | pisca, olhos crescem |
| Clique no Mochi | esmaga + fica irritado |
| Clique 3 vezes rápido | 😵‍💫 tonto por alguns segundos |
| Arraste um arquivo para a ilha | transforma-se numa caixa e o engole |
| Arraste o Mochi para uma janela *(macOS)* | anexa-a como contexto |
| Clique no nome do modelo acima da caixa de chat *(macOS)* | troca de provedor de IA ou modelo |

## Como funciona

**macOS**

- **Ilha**: um `NSPanel` sem bordas abraçando o notch, impulsionado por uma pequena máquina de estados (`hidden → petit → home`).
- **Personagem**: desenhado em `Canvas` + `TimelineView` do SwiftUI a 60 fps — corpo squircle, olhos projetados numa esfera, animações de mola. Sem Rive, sem Lottie, sem imagens.
- **Claude Code**: um pequeno script `nb-hook` recebe eventos de hook e os encaminha via um socket Unix para o app. Para aprovações, ele espera seu clique, e então responde ao hook.
- **Integrações**: pollers leves, pausados quando nada está observando.
- **Pílulas declaradas**: `PillCatalog.swift` é a única fonte da verdade — cada pílula (ferramentas de código, agentes, provedores de IA, serviços) é declarada lá com seu ID, cor e categoria.
- **Sons**: 28 WAVs curtos tocados por meio de `AVAudioPlayer`s pré-carregados.

O app para macOS é nativo em Swift 6 / SwiftUI / AppKit com **zero dependências de terceiros**.

**Windows**

- Um app [Tauri 2](https://tauri.app) (Rust + TypeScript): a ilha é uma janela transparente, sempre no topo (always-on-top), que nunca rouba o foco; Mochi é desenhado em Canvas 2D com as mesmas formas, tempos e sons que no Mac.
- Os hooks do Claude Code passam por um minúsculo `IA MOTION-hook.exe` e um named pipe; as chaves vivem no Windows Credential Manager.
- Detalhes e diferenças em [`windows/README.md`](windows/README.md).

**Linux**

- O mesmo app Tauri que o do Windows. No Wayland a ilha é um overlay do gtk-layer-shell ancorado na borda superior, e click-through é sua região de input.
- Os hooks do Claude Code passam pelo mesmo `IA MOTION-hook`, sobre um socket Unix em `$XDG_RUNTIME_DIR`; as chaves vivem no Secret Service.

## Contribuindo

Issues e PRs são muito bem-vindos — novas integrações, novos emotes, novos sons, correções de bugs. Veja [CONTRIBUTING.md](CONTRIBUTING.md).

## Créditos

Construído por [Louis Raillé](https://louisraille.fr) com Claude Code.
Inspirado pelos conceitos de companheiros de notch compartilhados por estúdios de design — este projeto é independente e não afiliado a nenhum deles.

## Licença

- **Código:** [MIT](LICENSE) — use-o, faça fork, aprenda com ele, apenas mantenha o aviso de direitos autorais.
- **Nome, personagem Mochi, ícone, sons e mídia:** © Louis Raillé, todos os direitos reservados — veja [LICENSE-ASSETS.md](LICENSE-ASSETS.md). Vai lançar seu próprio fork? Dê a ele seu próprio nome e personagem.

<div align="center">

**Se o Mochi te fez sorrir, uma ⭐ ajuda muito.**

[Site](https://louis-cfm.github.io/IA MOTION/) · [Privacidade](https://louis-cfm.github.io/IA MOTION/privacy.html) · [Termos](https://louis-cfm.github.io/IA MOTION/terms.html) · [Suporte](https://louis-cfm.github.io/IA MOTION/support.html)

</div>
