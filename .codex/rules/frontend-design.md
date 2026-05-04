# Frontend Design Rule

Use this rule when changing user-visible UI, CSS, layout, component composition, loading states, prompts, empty/error states, or visual assets in this Dioxus Authentication workspace.

## Direction

- Keep Template Project docs and Generated Project starter docs separate; visible app UI should remain generic unless a specific generated-project feature asks otherwise.
- Keep the template quiet, readable, and easy to repopulate.
- Favor reusable app structure over product-specific decoration.
- Preserve the direct authentication demo shape unless a future spec changes it.
- Keep copy and imagery generic; `Documentation/Images/Screenshot01.png` and `Documentation/Images/Infographic01.png` are replaceable slots.

## Layout

- Keep the web and desktop demo layouts visually consistent.
- Use dense but comfortable spacing and stable dimensions for authentication controls.
- Avoid nested cards and avoid decorative orbs/blobs.
- Make text fit in controls at desktop and mobile widths.

## Verification

- For browser-visible changes, run the real web app and inspect the result when practical.
- If the web server is already running on the target port, stop it and restart it before trusting the browser result.
