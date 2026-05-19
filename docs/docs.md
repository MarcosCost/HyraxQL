<div align="center">
  <img src="https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/database.svg" width="80" height="80" />
  <h1>HyraxQL</h1>
  <p><strong>A fast, lightweight, and modern database explorer.</br>Built in rust, for a fast and lightweight experience</strong></p>
</div>

<hr />

<div style="background-color: #1e1e2e; border: 1px solid #45475a; border-radius: 8px; padding: 20px; margin-bottom: 25px;">
  <h2 style="margin-top: 0; color: #fab387; border-bottom: none;">🚀 Getting Started</h2>
  <p>HyraxQL offers a seamless transition between traditional CLI usage and a powerful interactive shell.</p>
  
  <div style="display: flex; gap: 15px; margin-top: 15px;">
    <div style="flex: 1; background-color: #313244; padding: 15px; border-radius: 6px;">
      <h3 style="margin-top: 0; font-size: 1rem; color: #89b4fa;">Direct CLI</h3>
      <code>hyraxql connect -t postgres -u user -d db_name</code>
      <p style="font-size: 0.85rem; margin-bottom: 0;">Connect and start exploring immediately.</p>
    </div>
    <div style="flex: 1; background-color: #313244; padding: 15px; border-radius: 6px;">
      <h3 style="margin-top: 0; font-size: 1rem; color: #a6e3a1;">Interactive TUI</h3>
      <code>hyraxql</code>
      <p style="font-size: 0.85rem; margin-bottom: 0;">Start the shell and connect later.</p>
    </div>
  </div>
</div>

<h2 style="color: #cba6f7;">🛠 Global Flags</h2>
<table style="width: 100%; border-collapse: collapse; background-color: #1e1e2e; border-radius: 8px; overflow: hidden;">
  <thead style="background-color: #313244; color: #cdd6f4;">
    <tr>
      <th style="padding: 12px; text-align: left; border-bottom: 1px solid #45475a;">Flag</th>
      <th style="padding: 12px; text-align: left; border-bottom: 1px solid #45475a;">Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="padding: 12px; border-bottom: 1px solid #313244;"><code>-v, --verbose</code></td>
      <td style="padding: 12px; border-bottom: 1px solid #313244;">Enables debug logging for connection troubleshooting.</td>
    </tr>
    <tr>
      <td style="padding: 12px; border-bottom: 1px solid #313244;"><code>-h, --help</code></td>
      <td style="padding: 12px; border-bottom: 1px solid #313244;">Displays help information for CLI subcommands.</td>
    </tr>
    <tr>
      <td style="padding: 12px;"><code>-V, --version</code></td>
      <td style="padding: 12px;">Shows the current version of HyraxQL.</td>
    </tr>
  </tbody>
</table>

<br />

<h2 style="color: #f38ba8;">🐚 TUI Command Reference</h2>

<details style="background-color: #1e1e2e; border: 1px solid #45475a; border-radius: 8px; margin-bottom: 10px; padding: 10px;">
  <summary style="cursor: pointer; font-weight: bold; color: #89b4fa;">🔗 connect</summary>
  <div style="padding-top: 10px; color: #cdd6f4;">
    <p>Establishes a connection to a database instance.</p>
    <strong>Usage:</strong> <code>connect -t &lt;TYPE&gt; -u &lt;USER&gt; -d &lt;DB NAME&gt; -w &lt;PASSWORD&gt; -h &lt;HOST&gt; -p &lt;PORTS&gt;</code><br />
    <strong>Example:</strong> <code>connect -t "postgres" -u "marcos" -d "hyraxData"</code>
    <div style="margin-top: 10px; padding: 8px; background-color: #313244; border-left: 3px solid #89b4fa; border-radius: 4px;">
      💡 Note: Only the Type and Database name flags are obrigatory (All other flags, except password, default to something), however User and Ports are likelly not going to match your specific needs unless specified.
    </div>
    <div style="margin-top: 10px; padding: 8px; background-color: #313244; border-left: 3px solid #89b4fa; border-radius: 4px;">
      💡 Note: Sqlite connections only take Type and Database name, all other flags will be ignored
    </div>
  </div>
</details>

<details style="background-color: #1e1e2e; border: 1px solid #45475a; border-radius: 8px; margin-bottom: 10px; padding: 10px;">
  <summary style="cursor: pointer; font-weight: bold; color: #f9e2af;">🧹 clear</summary>
  <div style="padding-top: 10px; color: #cdd6f4;">
    <p>Wipes the terminal screen for a fresh workspace.</p>
    <strong>Usage:</strong> <code>clear</code>
  </div>
</details>

<details style="background-color: #1e1e2e; border: 1px solid #45475a; border-radius: 8px; margin-bottom: 10px; padding: 10px;">
  <summary style="cursor: pointer; font-weight: bold; color: #eba0ac;">🚪 exit</summary>
  <div style="padding-top: 10px; color: #cdd6f4;">
    <p>Gracefully shuts down the HyraxQL session.</p>
    <strong>Usage:</strong> <code>exit</code>
  </div>
</details>

<br />

<div style="background-color: #181825; padding: 15px; border-radius: 8px; border: 1px dashed #45475a;">
  <h3 style="margin-top: 0; font-size: 1.1rem; color: #cdd6f4;">⌨️ Keyboard Shortcuts</h3>
  <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px;">
    <div><kbd>↑</kbd> / <kbd>↓</kbd> History Navigation</div>
    <div><kbd>Ctrl</kbd> + <kbd>C</kbd> Interrupt / Exit</div>
    <div><kbd>Ctrl</kbd> + <kbd>D</kbd> EOF / Exit</div>
    <div><kbd>Tab</kbd> Autocomplete (Upcoming)</div>
  </div>
</div>

<br />

<h2 style="color: #a6e3a1;">🔌 Supported Engines</h2>
<div style="background-color: #1e1e2e; padding: 15px; border-radius: 8px;">
  <ul style="list-style: none; padding: 0; margin: 0;">
    <li style="margin-bottom: 8px;">🔹 <strong>PostgreSQL</strong>: <code>postgres://user:pass@host:5432/db</code></li>
    <li style="margin-bottom: 8px;">🔸 <strong>MySQL / MariaDB</strong>: <code>mysql://user:pass@host:3306/db</code></li>
    <li style="margin-bottom: 8px;">📦 <strong>SQLite</strong>: <code>sqlite://path/to/db.sqlite</code></li>
  </ul>
  <div style="margin-top: 15px; padding: 10px; background-color: #313244; border-left: 4px solid #a6e3a1; border-radius: 4px; color: #cdd6f4;">
    <strong>🔜 ROADMAP:</strong> Future support for MongoDB, Cassandra, and other NoSQL databases is planned.
  </div>
</div>

<br />

<div style="background-color: #1e1e2e; border: 1px solid #f38ba8; border-radius: 8px; padding: 15px; margin-bottom: 25px;">
  <h2 style="margin-top: 0; color: #f38ba8; border-bottom: none; font-size: 1.2rem;">⚠️ Error Handling</h2>
  <p style="margin-bottom: 0;">If a connection fails, HyraxQL will display the specific error returned by the database driver (e.g., authentication failure, unreachable host) and return you to the interactive prompt so you can retry with corrected credentials.</p>
</div>

<br />

<div style="text-align: center; color: #6c7086; font-size: 0.9rem;">
  <p>HyraxQL is licensed under the MIT License.</p>
</div>
