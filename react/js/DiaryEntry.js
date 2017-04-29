import React from 'react'
import { Link } from 'react-router'

import {submitNewComment} from './requests.js'
const { array, func, string, number } = React.PropTypes

const DiaryEntry = React.createClass({

  propTypes: {
    id: number,
    creation_date: string,
    creation_time: string,
    title: string,
    body: string,
    comments: array,
    reload: func
  },

  getInitialState () {
    return {
      commentBody: ''
    }
  },

  handleCommentInput (event) {
    this.setState({commentBody: event.target.value})
  },

  handleNewCommentSubmit (event) {
    event.preventDefault()
    // TODO: comment validation
    let commentBody = this.state.commentBody
    submitNewComment(this.props.id, commentBody).then(msg => {
      // reload the parent DiaryDetails
      this.props.reload()
    })
  },

  render () {
    // parse the date to a Date object
    const date = new Date(this.props.creation_date + ' ' + this.props.creation_time)
    const shortMonth = date.toLocaleString('en-us', { month: 'short' })
    const year = date.getFullYear()
    const timeStr = `${date.getHours()}:${date.getMinutes()}`

    if (this.props.isMetaInfo) {
            // <h2><a href={`/#/entry/${this.props.id}`}>{this.props.title}</a></h2>
      let borderStyle = {
        width: '70%',
        margin: '0 auto',
        borderBottom: '1px solid white'
      }
      // TODO: Don't build URL here dumbass
      return (
        <Link to={`/entry/${this.props.id}`}>
          <div className='diary-entry hvr-grow'>
            <header className='diary-header'>
              <h2 className='diary-title'>{this.props.title}</h2>
              <div style={borderStyle} />
              <p className='diary-date'>{`${timeStr} ${date.getDate()} ${shortMonth} ${year}`}</p>
            </header>

            <div className='diary-content'>
              <p>{this.props.body}</p>
            </div>

            <footer className='diary-footer'>
              <p className='diary-comments-count'>10 comments :)</p>
            </footer>
          </div>
        </Link>
      )
    }
    // if we're here, this DiaryEntry must be called from DiaryDetails
    // map the user friendly dates to the article comments
    this.props.comments.map((comment) => {
      const commentDate = new Date(comment.creation_date + ' ' + comment.creation_time)
      comment.shortMonth = date.toLocaleDateString('en-us', { month: 'short' })
      comment.day = commentDate.getDate()
      comment.year = commentDate.getFullYear()
      comment.timeStr = `${commentDate.getHours()}:${commentDate.getMinutes()}`
      comment.dateString = `${comment.shortMonth} ${comment.day} ${comment.year} - ${comment.timeStr}`
    })
       /*<form onSubmit={this.handleNewCommentSubmit}>
            <div className='article-comment'>
              <textarea name='commentBody' className='new-comment' onChange={this.handleCommentInput} />
              <button type='submit'> COMMENT </button>
            </div>
          </form>*/
    // TODO: Expand style width on more comments!
    return (
      <section className='diary-details'>
        <div className='diary-details-header'>
          <h1 className='diary-details-title'>{this.props.title}</h1>
          <h3 className='diary-details-date'>{`${timeStr} ${date.getDate()} ${shortMonth} ${year}`}</h3>
        </div>
        <div className='diary-details-content'>
          <p>{this.props.body}</p>
        </div>
        <div className='diary-details-comments'>
          {this.props.comments.map((comment) => {
            return (
              <div className='diary-details-comment' key={comment.id}>
                <div className='diary-details-comment-header'>
                  <h3 className='comment-author'>Netherblood</h3>
                </div>
                <div className='diary-details-comment-content'>
                  <p>{comment.body}</p>
                </div>
                <div className='diary-details-comment-footer'>
                  <p>{comment.dateString}</p>
                </div>
              </div>
            )
          })}
        </div>

        <div className='diary-details-new-comment'>
          <form className='diary-details-new-comment-form' onSubmit={this.handleNewCommentSubmit}>
            <textarea className='new-comment-content' name='commentBody' onChange={this.handleCommentInput} />
            <button type='submit' className='new-comment-submit'>Comment</button>
          </form>
        </div>
      </section>
    )
  }
})


export default DiaryEntry
