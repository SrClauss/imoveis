'use client';
import { useState } from "react";
import AlertDialog from "./components/alertDialog";
import DropFile from "./components/dropFile";


export default function Home() {
  const [hadImoveis, setHadImoveis] = useState(false);
  const [hadCliente, setHadCliente] = useState(false);
  const [hadVisitas, setHadVisitas] = useState(false);
  const [hadZap, setHadZap] = useState(false);

  const [zap, setZap] = useState(null);




  return (
    <div className="container">

      <div className="flex w-full justify-center">
        <img src="img/casa_alta.png" alt="Casa Alta Imoveis" width={'14%'} />

      </div>

      <DropFile onFileSubmit={e=>setZap(e)}  />
 
      <div className="flex w-full justify-center mt-5 ">
        <button id="btn-criterios" className="bg-amber-900 hover:bg-amber-800 text-white font-bold py-2 px-4 rounded w-full mx-10">Configurar Criterios</button>
      </div>
      

    </div >
  )
}
