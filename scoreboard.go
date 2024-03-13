package main

import (
	"context"
	"sync"
)

type CompressionRaio struct {
	line      int
	character int
}

func NewScoreboard(ctx context.Context) *Scoreboard {
	sb := &Scoreboard{
		lock:       sync.RWMutex{},
		data:       make(map[string]CompressionRaio),
		addSubs:    make(chan chan<- ScoreboardData),
		removeSubs: make(chan chan<- ScoreboardData),
		notify:     make(chan struct{}, 10),
	}
	go sb.Run(ctx)
	return sb
}

type ScoreboardData = map[string]CompressionRaio

type Scoreboard struct {
	lock       sync.RWMutex
	data       ScoreboardData
	subs       []chan<- ScoreboardData
	addSubs    chan (chan<- ScoreboardData)
	removeSubs chan (chan<- ScoreboardData)
	notify     chan struct{}
}

func (sb *Scoreboard) Subscribe(c chan<- ScoreboardData) {
	sb.addSubs <- c
}

func (sb *Scoreboard) Unsubscribe(c chan<- ScoreboardData) {
	sb.removeSubs <- c
}

func (sb *Scoreboard) Run(ctx context.Context) {
	for {
		select {
		case c := <-sb.addSubs:
			sb.subs = append(sb.subs, c)
		case c := <-sb.removeSubs:
			for i, s := range sb.subs {
				if s == c {
					sb.subs = append(sb.subs[:i], sb.subs[i+1:]...)
					break
				}
			}
		case <-sb.notify:
			for _, c := range sb.subs {
				c <- sb.data
			}
		case <-ctx.Done():
			return
		}
	}
}

func (sb *Scoreboard) Register(user string, newRatio CompressionRaio) (changed bool) {
	sb.lock.Lock()
	defer sb.lock.Unlock()
	d, ok := sb.data[user]
	changed = !ok || d.line < newRatio.line || (d.line == newRatio.line && d.character < newRatio.character)
	if changed {
		sb.notify <- struct{}{}
		sb.data[user] = newRatio
	}
	return
}

func (sb *Scoreboard) Get() ScoreboardData {
	sb.lock.RLock()
	defer sb.lock.RUnlock()
	return sb.data
}
