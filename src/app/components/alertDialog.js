import React from 'react';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';

import DialogTitle from '@mui/material/DialogTitle';
import { useState, useEffect } from 'react';
export default function AlertDialog({ open, onClose, onSend, sheets }) {

    const [selectedSheet, setSelectedSheet] = useState();


    useEffect(() => {

        setSelectedSheet(sheets[0]);


    }, [sheets]);

    const handleClose = () => {
        onClose(false);
    };
    const handleSend = () => {

        console.log(sheets[0]);
        onSend(selectedSheet);
        onClose(false);
    };




    return (
        <div>

            <Dialog
                open={open}
                onClose={handleClose}
                aria-labelledby="alert-dialog-title"
                aria-describedby="alert-dialog-description"
            >
                <DialogTitle id="alert-dialog-title">{"Selecione a planilha que a ser inserida"}</DialogTitle>
                <DialogContent>

                   <div>
                    <select onChange={(e)=>setSelectedSheet(e.target.value)}>

                        {sheets.map((sheet, index) => {
                            return <option key={index} value={sheet}>{sheet}</option>
                        })}



                    </select>
                        
                
                   </div>
                    
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleClose}>Cancelar</Button>
                    <Button onClick={handleSend} autoFocus>
                        Enviar
                    </Button>
                </DialogActions>
            </Dialog>
        </div>
    );
}