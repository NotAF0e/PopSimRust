from textual.app import App
from textual.widgets import Placeholder


class MainApp(App):

    async def on_mount(self) -> None:
        await self.view.dock(Placeholder(name="header"), edge="top", size=3)
        await self.view.dock(Placeholder(name="footer"), edge="bottom", size=3)
        await self.view.dock(Placeholder(name="stats"), edge="left", size=40)
        await self.view.dock(Placeholder(name="message"), edge="right", size=40)
        await self.view.dock(Placeholder(name="grid"), edge="top")


MainApp.run()
