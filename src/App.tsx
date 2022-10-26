import "./App.css";
import { Licenses } from './modules/LicensesView'
import { MainView } from './modules/MainView'
import { BrowserRouter, Routes, Route } from "react-router-dom";

function App() {
    return (

       <BrowserRouter>
         <Routes>
             <Route path={`/`} element={
                 <div className="App">
                    <MainView />
                </div>
              } />
             <Route path={`/licenses/`} element={<Licenses />} />
         </Routes>
      </BrowserRouter>
    );
}

export default App;
