package main

import "sync"

type CompressionRaio struct {
	line      int
	character int
}

func NewScoreboard() Scoreboard {
	return Scoreboard{
		lock: sync.RWMutex{},
		data: make(map[string]CompressionRaio),
	}
}

type ScoreboardData = map[string]CompressionRaio

type Scoreboard struct {
	lock sync.RWMutex
	data ScoreboardData
}

func (sb *Scoreboard) Register(user string, newRatio CompressionRaio) (changed bool) {
	sb.lock.Lock()
	defer sb.lock.Unlock()
	d, ok := sb.data[user]
	changed = !ok || d.line < newRatio.line || (d.line == newRatio.line && d.character < newRatio.character)
	if changed {
		sb.data[user] = newRatio
	}
	return
}

func (sb *Scoreboard) Get() ScoreboardData {
	sb.lock.RLock()
	defer sb.lock.RUnlock()
	return sb.data
}
