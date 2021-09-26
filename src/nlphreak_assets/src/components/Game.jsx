import React, { useState, useEffect } from 'react';

import { makeStyles } from "@material-ui/core/styles";
import Paper from "@material-ui/core/Paper";
import Grid from "@material-ui/core/Grid";
const useStyles = makeStyles((theme) => ({

}));


//getGame



const Game = ({ actor, id }) => {
    const classes = useStyles();


    const [gameData, setGameData] = useState("")
    const [ready, setReady] = useState(false)
    const [time, setTime] = useState(Date.now());

    useEffect(async () => {
        const self = await actor.getGame(id);
        console.log(self)
        setGameData(self)
        setReady(true)
    }, [id, time])



    useEffect(() => {
        const interval = setInterval(() => setTime(Date.now()), 5000);
        return () => {
            clearInterval(interval);
        };
    }, []);

    const [Q1, setQ1] = useState("")
    const [Q2, setQ2] = useState("")
    const [Q3, setQ3] = useState("")

    const updateQuestions = async (event) => {
        event.preventDefault();

        const allQuestions = [Q1, Q2, Q3]

        console.log(allQuestions)

        const x = await actor.addQuestions(parseInt(id), allQuestions);
        console.log(x)

    }

    return (
        <div>
            <p> Keywords: </p>
            <Grid container spacing={1}>
                {ready ? <> {gameData.keywords.map((k) => <Grid item classes={classes}><Paper><b>{k}</b></Paper></Grid>)}</> : <></>}
            </Grid>

            <p> Try to make the AI use these keywords by providing 3 prompts below (5-10 words each, don't use the keywords) </p>

            <p> You asked: </p>
            {ready && (gameData.questions.length > 0) ? <>
                <Grid container spacing={1}>
                    {ready ? <> {gameData.questions.map((k) => <Grid item classes={classes}><Paper><b>{k}</b></Paper></Grid>)}</> : <></>}
                </Grid>
            </> : <>
                <form onSubmit={updateQuestions}>
                    <input name="q1" onChange={(e) => setQ1(e.target.value)}></input>
                    <input name="q2" onChange={(e) => setQ2(e.target.value)}></input>
                    <input name="q3" onChange={(e) => setQ3(e.target.value)}></input>
                    <button type="submit">Submit</button>
                </form>
            </>}


            <p> AI answered: </p>

            {ready && (gameData.answers.length > 0) ? <>
                <Grid container spacing={1}>
                    {ready ? <> {gameData.answers.map((k) => <Grid item classes={classes}><Paper elevation="20"><b>{k}</b></Paper></Grid>)}</> : <></>}
                </Grid>

                <h1>Score: {gameData.score}</h1>
            </> : <></>}





        </div>
    )
}

export default Game
